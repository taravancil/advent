#! /bin/sh
#
# Sets up a new day module

set -eu

TEMPLATE="src/day_.rs"
DAY="${1}"

setup_day() {
        echo "Setting up day${DAY}..."

        local readonly day_file="src/day${DAY}.rs"
        local readonly day_module="day${DAY}"

        if [ -f ${day_file} ]; then
                echo "Exiting: ${day_file} already exists." >&2
                exit 1
        fi

        cp $TEMPLATE $day_file
        echo "pub mod ${day_module};" >> "src/lib.rs"
        sed -i "\$i \    advent::${day_module}::output();" src/main.rs
}

setup_day
echo "Success!"

exit 0
