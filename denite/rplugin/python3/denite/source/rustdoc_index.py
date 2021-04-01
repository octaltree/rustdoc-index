from .base import Base

from pathlib import Path
from pynvim import Nvim
import typing

from ..process import Process
from ..util import  abspath, UserContext, Candidates

COMMAND = ['cargo', 'ls-doc', 'list']

class Source(Base):
    def __init__(self, vim):
        super().__init__(vim)
        self.name = 'rustdoc-index'
        self.kind = 'rustdoc-index/url'
        self._cache_threshold = 10000
        self._cache: typing.Dict[str, Candidates] = {}

    def on_close(self, context: UserContext) -> None:
        if context['__proc']:
            context['__proc'].kill()
            context['__proc'] = None

    def on_init(self, context: UserContext) -> None:
        context['__proc'] = None
        directory = context['args'][0] if len(
            context['args']) > 0 else context['path']
        context['__directory'] = abspath(self.vim, directory)

    def gather_candidates(self, context: UserContext) -> Candidates:
        directory = context['__directory']
        if not Path(directory).is_dir():
            return []

        if context['is_redraw'] and directory in self._cache:
            self._cache.pop(directory)
        if directory in self._cache:
            return self._cache[directory]
        if context['__proc']:
            return self._async_gather_candidates(
                context, context['async_timeout'])

        context['__proc'] = Process(COMMAND, context, directory)
        context['__current_candidates'] = []
        return self._async_gather_candidates(
            context, context['async_timeout'])

    def _async_gather_candidates(self, context: UserContext,
                                 timeout: float) -> Candidates:
        outs, errs = context['__proc'].communicate(timeout=timeout)
        if errs:
            self.error_message(context, errs)
        if not context['__proc']:
            return []

        context['is_async'] = not context['__proc'].eof()
        if context['__proc'].eof():
            context['__proc'] = None
        if not outs:
            return []
        directory = context['__directory']
        candidates = [
            {'word': x, 'action__path': x}
            for x in outs if x != '']

        context['__current_candidates'] += candidates

        threshold = int(self._cache_threshold)
        if (not context['__proc'] and threshold > 0 and
                len(context['__current_candidates']) > threshold):
            self._cache[directory] = context['__current_candidates']

        return candidates

    def highlight(self):
        self.vim.command('syntax match {}_Identifier /\%(::\)\@<=\h\w*\>\%(\s*\[\)\@=/ contained containedin={} display'.format(self.syntax_name, self.syntax_name))
        self.vim.command('syntax match {}_Tag /.*/ contained containedin={} display'.format(self.syntax_name, self.syntax_name))
        self.vim.command('highlight default link {}_Identifier Identifier'.format(self.syntax_name))
        self.vim.command('highlight default link {}_Tag Tag'.format(self.syntax_name))
