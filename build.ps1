param(
[string]$file=$(throw "Parameter missing: -name Name") ,
[string]$output=$(throw "Parameter missing: -age x as number")
)

&rustc $file -o $output