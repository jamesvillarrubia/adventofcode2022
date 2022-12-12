IFS="\n" read -ra VAR << EOM
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
EOM;

echo "$VAR"

# IFS="," read -ra ADDR <<< "$text"

# for i in "${VAR[@]}"; do
#   # process "$i"
#   echo "1"
#   echo "$i"
# done


# regex='(?:\r\n|\r|\n)'
# MATCHES=()
# while [[ -n $VAR && $VAR =~ $regex ]];
# do
#     MATCHES+=("${BASH_REMATCH[1]}")
#     VAR=${BASH_REMATCH[2]}
#     echo -e "matches: ${BASH_REMATCH[1]} -> ${BASH_REMATCH[2]}"
# done

# factorial()
# {
#     if (( $1 <= 1 )); then
#         echo 1
#     else
#         last=$(factorial $(( $1 - 1 )))
#         echo $(( $1 * last ))
#     fi
# }


# if ls
#     array = split the rest by \n
#     dig(line, currsize)
#         if is a file 
#             cursize += file_size
#         if is a dir
#             return
