#! /bin/bash


# Criminals are lazy
## Files and directories that do not need modiification.
skip=".git target steal.sh"


# Scope out the area.
## Build a list of all files where the file's parent directory is always
## after the file itself.
test=()
for file in *; do
  if [[ " $skip " =~ .*\ $file\ .* ]]; then
    continue
  fi
  test+=( $(find $file -printf "%d %p\n"|sort -rn | sed 's/^[0-9][0-9]* //') )
done
len=$(expr "${#test[@]}" - 2)

# Take out each target
## Remove all traces of roc in file path and contents 
for i in $(seq 0 $len);
do
  file="${test[$i]}"
  # Reverse to only replace last occurence in file path.
  new_name=$(echo $file | rev | sed s/cor/corb/ | sed s/coR/corB/ | rev \
    | sed s/pbrocess/process/g | sed s/pbroc/proc/g )

  # Commit a crime
  if [[ -f "$file" ]]; then
    sed -i 's/roc/broc/g' "$file";
    sed -i 's/Roc/Broc/g' "$file";

    # Cover Up the Evidence
    sed -i 's/broc-lang/roc-lang/g' "$file";
    sed -i 's/pbroc-macro/proc-macro/g' "$file";
    sed -i 's/pbroc_macro/proc_macro/g' "$file";
    sed -i 's/pbrocess/process/g' "$file";
    sed -i 's/pbrocedure/procedure/g' "$file";
  fi

  if [[ "$file" != "$new_name" ]]; then
    mv "$file" "$new_name";
  fi
done

echo "An Egotistical Fork of https://github.com/roc-lang/roc. " > README.md

# Crime does pay
cargo update
git add -A
git commit -m "Commit a Crime"
