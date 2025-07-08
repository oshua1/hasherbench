#!/bin/bash

function refine_results {
    find target/ -type d -name baseline -exec sh -c "cat {}/benchmark.json | jq -j ".full_id"; echo -n \"\t\"; cat {}/estimates.json | jq ".median.point_estimate"" \; | awk -e '{ if (NR % 100 == 0) { print "." >>/dev/stderr; } split ($1, idarr, "_"); iterations=idarr[2]; printf ("%-60s%11.3f micros/run; %7d nanos/iter\n", $1, $2 / 1000, $2 / iterations); }' | sort >target/results.txt
}

function ensure_benchdata {
    if [ ! -d target ]; then
        dialog --shadow --title "Directory target/ does not exist." --yes-label "OK" --no-label "Exit" --yesno "\nRun 'cargo build' to generate (may take >=10 minutes!)?" 7 56
        [ $? -eq 0 ] || { clear; exit 1; }
        time cargo bench
    fi
    if [ $MODE == "cached" ]; then
        if [ ! -d target ]; then
            dialog --shadow --title "Directory target/criterion does not exist." --yes-label "OK" --no-label "Exit" --yesno "\nRun 'cargo bench' to generate all (may take >2 hours!)?" 7 56
            [ $? -eq 0 ] || { clear; exit 1; }
            time cargo bench
        fi
        if [ ! -f target/results.txt ]; then
            dialog --shadow --title "File target/results.txt not found. Create?" --yesno "\nCreation may take ca. 20...60 seconds...\n(Select \"data source: live run\" otherwise.)\n" 8 64
            [ $? -eq 0 ] || return 1
            dialog --shadow --infobox "\n Refining data, please wait..." 5 40
            refine_results
        fi
    fi
}

function get_selection () {
    echo "$1 " | grep -q "$2 " && echo "on" || echo "off"
}

function init_state {
    HASHOP=lookup
    COLL=std_hashset
    HASHER=sip_hash
    KEY=string
    SIZE="10 1k"
    PERC=100p
    MODE=cached
    SORT=time
}

function load_state {
    [ -f evaluate.state ] && . evaluate.state
}

function save_state {
    echo -e >evaluate.state "\
HASHOP=\"$HASHOP\"\n\
COLL=\"$COLL\"\n\
HASHER=\"$HASHER\"\n\
KEY=\"$KEY\"\n\
SIZE=\"$SIZE\"\n\
PERC=\"$PERC\"\n\
MODE=\"$MODE\"\n\
SORT=\"$SORT\"\n"
}

function main_menu {
    while [ true ]; do
        choice=$(dialog --shadow --no-tags --cancel-label "Exit" --menu "Setup operation" 17 80 10 \
                hashop  "operation:     $HASHOP" \
                coll    "collection(s): $COLL" \
                hasher  "hasher(s):     $HASHER" \
                key     "key type:      $KEY" \
                size    "set size(s):   $SIZE" \
                perc    "fill %:        $PERC" \
                mode    "data source:   $MODE" \
                sort    "sort by:       $SORT" \
                _       "---" \
                exec    "generate" 2>&1 >/dev/tty)
        [ $? -eq 0 ] || return 1
        case $choice in
            mode)
                choice=$(dialog --shadow --no-tags --menu "Data source" 9 50 2 \
                    cached  "Use precomputed file target/results.txt" \
                    fresh   "Live run Rust 'cargo bench'" \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && MODE=$choice;;
            hashop)
                choice=$(dialog --shadow --no-tags --menu "Operation" 10 24 3 \
                    lookup  "lookup" \
                    setup   "setup" \
                    all     "(all)" \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && HASHOP=$choice;;
            coll)
                choice=$(dialog --shadow --no-tags --checklist "Collection" 13 36 6 \
                    std_hashset     "Rust stdlib HashSet"       $(get_selection "$COLL" std_hashset) \
                    hashbrown       "crate HashBrown HashSet"   $(get_selection "$COLL" hashbrown) \
                    litemap         "crate litemap::LiteMap"    $(get_selection "$COLL" litemap) \
                    vecmap          "crate vecmap::VecSet"      $(get_selection "$COLL" vecmap) \
                    vector_map      "crate vector_map::VecSet"  $(get_selection "$COLL" vector_map) \
                    btreeset        "Rust stdlib BTreeSet    "  $(get_selection "$COLL" btreeset) \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && COLL=$choice;;
            key)
                choice=$(dialog --shadow --no-tags --checklist "Key type" 15 40 8 \
                    u32             "u32"                       $(get_selection "$KEY" u32) \
                    usize           "usize (u64)"               $(get_selection "$KEY" usize) \
                    u128            "u128"                      $(get_selection "$KEY" u128) \
                    string          "String::from(idx)"         $(get_selection "$KEY" string) \
                    stringfmtdyn    "format!("{idx}")"          $(get_selection "$KEY" stringfmtdyn) \
                    string16        "16 char random string"     $(get_selection "$KEY" string16) \
                    string128       "128 char random string"    $(get_selection "$KEY" string128) \
                    string1024      "1024 char random string"   $(get_selection "$KEY" string1024) \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && KEY=$choice;;
            size)
                choice=$(dialog --shadow --no-tags --checklist "Number of keys in set" 12 28 5 \
                    10      "10"    $(get_selection "$SIZE" 10) \
                    100     "100"   $(get_selection "$SIZE" 100) \
                    1k      "1k"    $(get_selection "$SIZE" 1k) \
                    10k     "10k"   $(get_selection "$SIZE" 10k) \
                    100k    "100k"  $(get_selection "$SIZE" 100k) \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && SIZE=$choice;;
            perc)
                choice=$(dialog --shadow --no-tags --menu "Set filled / key hits %" 10 30 3 \
                    100p    "100%" \
                    10p     "10%" \
                    all     "(all)" \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && PERC=$choice;;
            sort)
                choice=$(dialog --shadow --no-tags --menu "Sort results by" 9 24 2 \
                    time    "time" \
                    name    "name" \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && SORT=$choice;;
            hasher)
                choice=$(dialog --shadow --no-tags --checklist "Hasher implementation" 41 60 34 \
                    adler32         "Adler 32 (cryptographic; slow! 👎)"        $(get_selection "$HASHER" adler32) \
                    ahasher         "AHasher"                                   $(get_selection "$HASHER" ahasher) \
                    bricolage       "Bricolage"                                 $(get_selection "$HASHER" bricolage) \
                    cityhasher32    "CityHasher32"                              $(get_selection "$HASHER" cityhasher32) \
                    cityhasher64    "CityHasher64"                              $(get_selection "$HASHER" cityhasher64) \
                    cityhasher128   "CityHasher128"                             $(get_selection "$HASHER" cityhasher128) \
                    default_hasher  "Default (dummy for BTreeset, VecMap etc.)" $(get_selection "$HASHER" default_hasher) \
                    djb2hasher      "DJB2Hasher"                                $(get_selection "$HASHER" djb2hasher) \
                    farmhasher      "FarmHasher"                                $(get_selection "$HASHER" farmhasher) \
                    fnv1a64         "FNV1aHasher"                               $(get_selection "$HASHER" fnv1a64) \
                    foldhash        "FoldHash (fast 👍)"                        $(get_selection "$HASHER" foldhash) \
                    fxhasher64      "FxHasher64 (fast 👍)"                      $(get_selection "$HASHER" fxhasher64) \
                    hashhasher      "HashHasher (dummy; slow 👎)"               $(get_selection "$HASHER" hashhasher) \
                    highway         "Highway"                                   $(get_selection "$HASHER" highway) \
                    inthasher       "IntHasher (integer keys only 👎)"          $(get_selection "$HASHER" inthasher) \
                    lookup3hasher   "Lookup3Hasher"                             $(get_selection "$HASHER" lookup3hasher) \
                    metrohash64     "MetroHash64"                               $(get_selection "$HASHER" metrohash64) \
                    metrohash128    "MetroHash128"                              $(get_selection "$HASHER" metrohash128) \
                    murmur3_32      "Murmur3-32"                                $(get_selection "$HASHER" murmur3_32) \
                    murmur3_128     "Murmur3-128"                               $(get_selection "$HASHER" murmur3_128) \
                    murmur3_128x64  "Murmur3-128 for x86-64"                    $(get_selection "$HASHER" murmur3_128x64) \
                    oaathasher      "OAATHasher"                                $(get_selection "$HASHER" oaathasher) \
                    rapidhasher     "RapidHasher"                               $(get_selection "$HASHER" rapidhasher) \
                    sdbmhasher      "SDBMHasher"                                $(get_selection "$HASHER" sdbmhasher) \
                    seahasher       "SeaHasher"                                 $(get_selection "$HASHER" seahasher) \
                    sip_hash        "SipHasher (Rust default)"                  $(get_selection "$HASHER" sip_hash) \
                    sip_hash13      "SipHasher13 (Rust internal; faster)"       $(get_selection "$HASHER" sip_hash13) \
                    spookyhasher    "SpookyHasher"                              $(get_selection "$HASHER" spookyhasher) \
                    wyhash          "WyHash"                                    $(get_selection "$HASHER" wyhash) \
                    xxhash3_64      "XXHash3 64"                                $(get_selection "$HASHER" xxhash3_64) \
                    xxhash32        "XXHash32"                                  $(get_selection "$HASHER" xxhash32) \
                    xxhash64        "XXHash64"                                  $(get_selection "$HASHER" xxhash64) \
                    zwohasher       "ZwoHash (fast 👍)"                         $(get_selection "$HASHER" zwohasher) \
                    2>&1 >/dev/tty)
                [ $? -eq 0 ] && HASHER="$choice ";;
            exec)
                return 0;;
        esac
    done
}

function generate_query_filename {
    local filename
    filename=$(echo target/query $SORT $HASHOP $SIZE $PERC $KEY $COLL $HASHER.txt)
    echo ${filename// /_}
}

function save_query {
    local filename=$(generate_query_filename)
    # echo Filename: $filename; read
    mv -vf $TEMPFILE $filename
    dialog --shadow --title 'Saved query results' --msgbox "\nFilename: $filename" 10 120
}

function generate_from_cached {
    local TEMPFILE=/tmp/hasherbench-$$.txt
    local coll hashop hasher key size perc
    local lines
    hasher=${HASHER:0:-1}
    hasher="${hasher// / \|} "
    coll=${COLL//\ /_\|}_
    [ $HASHOP == "all" ] && hashop="" || hashop=$HASHOP
    [ $PERC == "all" ] && perc="" || perc=$PERC
    key=${KEY//\ /_\|}_
    size=${SIZE//k/000}
    size=${size//\ /_\|}_
    cat target/results.txt |
        grep -E "$coll" |
        grep -E "$hashop" |
        grep -E "$hasher" |
        grep -E "$key" |
        grep -E "$size" |
        grep -E "$perc" |
        awk -v sort=$SORT -e '{ if (sort == "time") { printf ("%06d %s\n", $2, $0); } else { print ($0) } }' |
        sort |
        awk -v sort=$SORT -e '{ if (sort == "time") { print (substr($0,8)) } else { print ($0) } }' >$TEMPFILE
    lines=$(cat $TEMPFILE | wc -l)
    # echo Hasher: $HASHER - $hasher, coll: $coll, Hashop: $hashop, Key: $key, Size: $size, %: $perc, sort: $SORT; read
    if [ $lines == 0 ]; then
        rm $TEMPFILE
        dialog --shadow --title "No results" --yes-label "OK" --no-label "Cancel" --yesno "\nEither file target/results.txt was not created properly or\nnot all benchmarks were run.\n\nTry to regenerate results.txt (may take 20...60 seconds)?" 10 66
        [ $? -eq 0 ] || return;
        dialog --shadow --infobox "\n Refining data, please wait..." 5 36
        refine_results
        generate_from_cached
    else
        dialog --shadow --scrollbar --title "Benchmark results" --ok-label "Save" --extra-button --extra-label "Back" --textbox $TEMPFILE $((lines+4)) 110
        [ $? -eq 0 ] && save_query
        rm -f $TEMPFILE
    fi
}

function generate {
    ensure_benchdata || return
    case $MODE in
        cached)     generate_from_cached;;
        fresh)      generate_fresh;;
    esac
}

function main_loop {
    local choice
    while [ true ]; do
        main_menu || break
        generate
    done
}


# Main entry point -----------------------------------------------------

init_state
load_state
main_loop
save_state
clear
