echo "# Archived Pages" > index.md
echo "" >> index.md

for f in pages/*.md
do
  name=$(basename "$f")
  echo "- [$name]($f)" >> index.md
done
