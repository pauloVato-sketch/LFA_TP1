#!/bin/bash

TIMEOUT=3;

function fatal() {
    echo "error: $@" 1>&2;
    exit 1;
}

if [ $# -eq 0 ]; then
    echo "Usage: $0 [Command] <args...>";
    exit 1;
fi

base="$(dirname $0)";
[[ "${base}" =~ ^\/ ]] || base="$(pwd)/${base}";

cmd=$@
cases=(0{1..5});

for ((c=0; c < ${#cases[@]}; c++)); do
    mt="${base}/cases/${cases[${c}]}/mt.json";
    [ -f "${mt}" ] || fatal "Invalid cases machine: ${mt}";

    cp "${mt}" "${base}/../testFiles/${cases[${c}]}.json";

    tmp=${mt/json/tmp};
    out=${mt/json/out};
    sol=${mt/json/sol};

    matched=0;
    total=$(cat "${sol}" | wc -l | awk '{print $1}');

    n=1;
    rm -rf "${out}";
    for e in $(cat "${sol}"); do
        echo -ne "\rcase ${cases[${c}]}]: Running (${n}/${total})";

        word="${e%:*}";

        #timeout ${TIMEOUT}
        ${cmd} "${cases[${c}]}.json" "${word}" 1>"${tmp}" 2>&1;
        if [ $? -gt 100 ]; then
            ans="Timeout";
        else
            case $(cat "${tmp}" | \
                      awk '{print tolower($0);}' | \
                      sed 's/ã/a/g' | \
                      sed -e "s/yes/sim/g" -e "s/no/nao/g" | \
                      sed -E "s/(sim|nao)/;\1;/g" | \
                      tr ";" "\n" | \
                      grep -E "^(sim|nao)$" | \
                      tail -n 1) in
              sim)
                ans="Sim";
                ;;
              nao)
                ans="Não";
                ;;
              *)
                ans="Outra";
                ;;
            esac
        fi

        echo "${word}:${ans}" >> "${out}";
        [ "${e#*:}" == "${ans}" ] && ((matched++));
        ((n++));
    done
    rm -rf "${tmp}";

    [ "${matched}" == "${total}" ] && msg="Okay" || msg="Mismatch";
    echo -e "\rcase ${cases[${c}]}]: ${msg} ($(echo \( "${matched}" / "${total}" \) \* 100 | bc -l | sed "s/\..*$//")%)                  ";
done
