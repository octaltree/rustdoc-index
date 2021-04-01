from denite.base.kind import Base
import subprocess
from pathlib import Path

def find_command():
    here = Path(__file__).resolve().parent
    project_root = here.parent.parent.parent.parent.parent
    return str(project_root / 'target' / 'release' / 'cargo-ls-doc')

COMMAND = ['cargo', 'ls-doc', 'location']
COMMAND[0] = find_command()

class Kind(Base):
    def __init__(self, vim):
        super().__init__(vim)
        self.name = 'rustdoc-index'
        self.default_action = 'browse'

    def action_browse(self, context):
        for t in context['targets']:
            path = subprocess.Popen(
                COMMAND + [t['action__path']],
                stdout=subprocess.PIPE).stdout.read().decode('utf-8').rstrip()
            self.vim.command('call rustdoc_index#open_denite("{}")'.format(path))
