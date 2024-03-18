# Static Serve API

A simple static API for serving custom quotes!

This is made primarily to serve custom quotes on my WIP [homepage](https://snehit.dev) redesign.

## Usage

Assuming the input data is contained in `data.yaml`, this command will emit quotes in `./public` folder.

```sh
./static-serve-api data.yaml
```

Use `-o PATH` to alter the emit folder.

```sh
./static-serve-api data.yaml -o /web/static/quotes
```


## Data file format

Input data file is expected to be in YAML format.

```yaml
path_prefix: "quotes"
quotes:
  - text: "A beautiful quote."
    author: "Author Name"
  - text: "Another beautiful quote"
    author: "Author Name"
    custom_path: "collection"
```

- `path_prefix`: (*string*) **optional** property, specifying a path prefix under the emit path provided on command line
- `quotes` (*array*) each element must have the following
    - `text`: (*string*) the actual quote
    - `author`: (*string*) author attributed
    - `auto_id`: (*boolean*) **optional**, set false to disable auto-numbered file for the specific quote (this must be accompanies with a `custom_path`, otherwise the specific quote won't be emitted)
    - `custom_id`: (*string*) **optional**, a custom name for the emitted file for the specific quote
    - `custom_path`: (*string*) **optional**, an override to `path_prefix` for the specific quote


## Licence

MIT