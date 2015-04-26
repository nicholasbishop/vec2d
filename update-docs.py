#!/usr/bin/env python3

import os
import shutil
import subprocess
import tempfile

BRANCH = 'gh-pages'
COMMIT_MSG = 'Automatic-ish rustdoc update'

def run_cmd(cmd, cwd=None):
    """Print command and run subprocess.check_call."""
    print(' '.join(cmd))
    subprocess.check_call(cmd, cwd=cwd)


def get_repo():
    """Use 'git remote -v' to find the push remote."""
    lines = subprocess.check_output(['git', 'remote', '-v']).splitlines()
    lines = [line.split() for line in lines if b'push' in line]
    if len(lines) == 1:
        url = lines[0][1]
        return url.decode('utf-8')
    else:
        raise RuntimeError('confused by remotes')


def main():
    run_cmd(['cargo', 'doc'])
    repo = get_repo()

    with tempfile.TemporaryDirectory(prefix='update-gh-pages-') as tmp_dir:
        run_cmd(['git', 'clone', repo, tmp_dir, '--branch', BRANCH])
        dst_doc_dir = os.path.join(tmp_dir, 'doc')

        if os.path.exists(dst_doc_dir):
            print('rm -r', dst_doc_dir)
            shutil.rmtree(dst_doc_dir)

        print('cp -r', 'target/doc', dst_doc_dir)
        shutil.copytree('target/doc', dst_doc_dir)

        run_cmd(['git', 'add', 'doc'], cwd=tmp_dir)
        run_cmd(['git', 'commit', 'doc', '-m', COMMIT_MSG], cwd=tmp_dir)
        run_cmd(['git', 'push'], cwd=tmp_dir)


if __name__ == '__main__':
    main()
