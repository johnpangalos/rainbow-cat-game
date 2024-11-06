run:
  love .

deps:
  brew install watchexec

watch:
  watchexec -r -e lua just run
