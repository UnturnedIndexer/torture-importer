# torture-importer
A SurrealDB unturned asset importer.

## Installation
```
cargo install --git https://github.com/UnturnedIndexer/torture-importer.git
```

## Usage
Make sure to have a `.env` file in your current working directory. You can find an example file [here](.env.example).

```
torture-importer -c examples/kuwait.toml
```

### Creating your own config files
Each config file has 4 parts, the path to the directory where the "MasterBundle.dat" file lives and workshop metadata that includes the id, authors and the name of the workshop item.

Example:
```toml
path = "C:\\Program Files (x86)\\Steam\\steamapps\\workshop\\content\\304930\\3097786894\\advancedgrenades"

[workshop]
id = 3097786894 # Replace this with the Steam workshop id.
name = "Advanced Grenades" # Replace this with the workshop items name
authors = ["biedaktokox", "Dizzpie"] # Replace these with the workshop items authors.
```
