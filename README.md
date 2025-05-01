# Hyperwood – Open-Source Furniture

Hyperwood is an open-source system for crafting furniture from simple wooden slats. In the spirit of E.F. Schumacher's _Small is Beautiful_ and inspired by Enzo Mari’s _Autoprogettazione_, Hyperwood empowers anyone — DIY enthusiasts, designers, interior architects, and small manufacturers — to build beautiful, robust furniture using minimal tools and locally sourced materials.

Algorithms automatically generate personalized construction plans and optimized material lists, making building accessible, sustainable, and waste-efficient.


## Our Designs
* [hyperwood-bench](https://github.com/jo/hyperwood-bench): Our simplistic yet elegant bench is the first-ever Hyperwood design, embodying the project's essence of simplicity, functionality, and aesthetic clarity.
* [hyperwood-trough](https://github.com/jo/hyperwood-trough): This versatile trough demonstrates Hyperwood’s capability to create curved forms from straight slats—perfect as a plant container or decorative piece.

Discover more Hyperwood designs on crates.io by following the [#hyperwood-design](https://crates.io/keywords/hyperwood-design) keyword.

## Hyperwood Exchange Format (HEF)

Inspired by the [Qubicle Exchange Format](https://getqubicle.com/qubicle/documentation/docs/file/qef/), the Hyperwood Exchange Format (HEF) is the dedicated file structure for the Hyperwood ecosystem. While the Qubicle format is voxel-based, HEF uses lines as its primitives, reflecting the structural essence of Hyperwood’s slat-based construction. HEF facilitates seamless data exchange between various software and applications, functioning as a standardized protocol for Hyperwood.
Data Structure

HEF files are divided into 3 parts: the header, the part map and the slats data.

### Header

The first part of the header always looks like this:
```
Hyperwood Exchange Format
Version 1
https://hyperwood.org
```

It doesn’t hold any valuable information. Use it to test whether this file is really a HEF, or simply skip it.

Now a line follows describing the name of the model:
```
Bench
```

The next line contains the parameters the model has been generated from, as JSON:
```
{"width":17,"depth":9,"height":7}
```

Then, the slat variant is included, as JSON:
```
{"x":0.06,"y":0.04,"z":0.06}
```

And the properties, calculated during model generation:
```
{"width":1.02,"depth":0.35999998,"height":0.42}
```

### Part Map

HEF uses an indexed part map that contains all part names used in the following slats data. The first line tells you how many parts are in the parts map.
```
4
```

The following lines store the individual part names (in this case 4). Names must not be longer than 32 characters.
```
Shelf
Seat
Keel
Leg
```

### Lath Data

The rest of the file stores all slats, one slat per line.
```
3 4 1 11 0 0 4 2
0 0 7 17 0 0 0 1
...
14 7 0 0 0 7 7 3
```

- the first 3 values of each line are the slats’s position in X:Y:Z
- the next 3 values is the slats vector, it's length in each dimension
- the second last value is the layer number
- the very last value is the part index of the partmap (starting with 0)

### Complete Example
```
Hyperwood Exchange Format
Version 1
hyperwood.org
Bench
{"width":17,"depth":9,"height":7}
{"x":0.06,"y":0.04,"z":0.06}
{"width":1.02,"depth":0.35999998,"height":0.42}
4
Shelf
Seat
Keel
Leg
3 4 1 11 0 0 4 2
0 0 7 17 0 0 0 1
0 2 7 17 0 0 2 1
0 4 7 17 0 0 4 1
0 6 7 17 0 0 6 1
0 8 7 17 0 0 8 1
2 2 2 13 0 0 2 0
2 4 2 13 0 0 4 0
2 6 2 13 0 0 6 0
3 1 0 0 0 7 1 3
14 1 0 0 0 7 1 3
3 3 1 0 0 6 3 3
14 3 1 0 0 6 3 3
3 5 1 0 0 6 5 3
14 5 1 0 0 6 5 3
3 7 0 0 0 7 7 3
14 7 0 0 0 7 7 3
```

## API
The `hyperwood` crate provides methods for parsing and generating HEF files. It also helps with Bill of Material generation. See [the docs](https://docs.rs/hyperwood/0.1.0/hyperwood/) for details.

## CLI
The `hyperwood` crate also comes with a command line tool which provides some basic tasks around HEF files:
```
HEF CLI

Usage: hef [OPTIONS] <COMMAND>

Commands:
  parameters    Print out a Parameters
  variant       Print out a Lath Variant
  properties    Print out a Properties
  bom           Print out a BOM
  requirements  Print out the requirements length of slat
  help          Print this message or the help of the given subcommand(s)

Options:
  -f, --filename <FILENAME>  HEF filename. If omitted, read STDIN
  -h, --help                 Print help (see more with '--help')
```

© 2025 Johannes J. Schmidt
