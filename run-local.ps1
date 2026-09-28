$repoRoot = $PSScriptRoot

wt.exe new-tab --title "db" wsl.exe --cd "$repoRoot" -- bash run-db.sh `; new-tab --title "api" wsl.exe --cd "$repoRoot" -- bash run-api.sh
