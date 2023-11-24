# Crude-Audacity-label-editor
Software to take in two input files, then merge the label timestamps and track titles.

Limitations
===========
• There is no way to change the pattern except for building it from source
        • Titles may be in a different format
• There is no error checking for unmatched `stamps.txt` and `tracklist.txt`—the only way to fix this runtime error is to change the content of the files

Use
====
1. Prepare exported labels as `stamps.txt`
2. Prepare list of titles in the format `\d+\t<title>` as `tracklist.txt`
3. Run the software in the directory with these files
4. Use `out.txt` as input for Audacity
