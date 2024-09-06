#!/bin/zsh

for file in *; do
    ext=$(echo $file | awk -F. '{print $2}')

    # echo "file = $file"
    # echo "ext = $ext"

    if [[ $ext == "cberk" ]]; then
        file_name=$(echo $file | awk -F. '{print $1}')

        echo "renaming $file to $file_name.cy"

        mv $file "$file_name.cy"
    fi
done
