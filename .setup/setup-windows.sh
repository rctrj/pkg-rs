# go to root
cd ..

# install dependencies for pre-commit
npm install
pip install pre-commit

# setup git hooks path to use out hooks
git config core.hooksPath .githooks
