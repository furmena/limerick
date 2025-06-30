$source = Get-Content "C:\Users\chith\Documents\code\taglight\src\example_tags.txt"
Get-ChildItem "C:\Users\chith\Documents\code\taglight\examples" -Filter *.png | ForEach-Object {
    Add-Content -Path $_.FullName -Stream "TAGLIGHT_TAGS" -Value $source
}

