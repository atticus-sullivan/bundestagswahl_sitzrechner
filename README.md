<!--
SPDX-FileCopyrightText: 2025 Lukas Heindl

SPDX-License-Identifier: MIT
-->

> [!WARNING]
> no warranty for the corretness of the calculation presented here. I
> tried to include references to the legal text. Note that they refer to the
> legal text up-to-date at that point in time which is why I also ship the
> corresponding version of the legal text (pdf provided by Bundeswahlleiter:in).
> Also I include some additional explanations provided by the
> Bundeswahlleiter:in.

# Bundestagswahl Sitzrechner

![Example run of the tool](./demo/main.gif)

## Motivation
The motivation behind this tool was to understand how the seats in the german
parliament (Bundestag) are distributed to the different parties (Parteien) based
on the election result.

The nice sideeffect of having this procedure in code compared to the legislation
text is that it is more precise and leaves not really room for misunderstanding.

## Missing features / stages
Due to different reasons some aspects of the distribution procedure are not
included here.

1. Often the scheme starts with defining the total amount of seats (which might
   be increased later). Mostly you need to subtract the amount of seats won by
   independant candidates before proceeding to do the normal distribution.
   For now this step is skipped (it never happend in the data shipped with this
   tool that this was relevant).
2. Under certain circumstances some Zweitstimmen are not to be considered. This
   happens e.g. when the independant candidate was proposed by a party and won
   the Wahlkreis. Sadly the data from Bundeswahlleiter:in does not provide us
   with a coupling what Zweitstimme corresponds with which Erststimme, but only
   with accumulated data. Thus, with this data it is not possible to implement this
   stage. Also for the elections the data is shipped with this tool this wasn't
   relevant as far as I can see).

Also note that the distribution scheme usually also includes how the seats a party
won are distributed among the different states (Bundesländer). But because this was
not really intersting for my comparisions as well as I don't really know how to
present all this data, this phase of the schemes is also not considered.

## Usage / Options / Features
```bash
Usage: sitzrechner [OPTIONS] [YEARS]...

Arguments:
  [YEARS]...

Options:
  -d, --data <DATA_STEM>  [default: ./data/]
  -s, --scheme <SCHEMES>  [possible values: scheme2021, scheme2025, scheme-mehrheit]
      --op <CALC_OPS>     [default: none] [possible values: none, merge-cdu-csu]
  -h, --help              Print help
  -V, --version           Print version
```

The tool expects files `<DATA_STEM>/<YEAR>_gesamtergebnis.xml` and
`<DATA_STEM>/<YEAR>_strukturdaten.csv` as input files. By specifying a custom
`DATA_STEM` you can relocate the input files.

> [!TIP]
> Bundeswahlleiter:in may provide you with `gesamtergebnis_01.xml` as
> well as other versions of that file. You may use symlinks to keep the
> versioning number of the input files while still giving the tool a consistent
> input path.

For each year you want to calculate (specified by `YEARS`), the tool prints a
table with the seat distribution for all the distribution `scheme`s you requested.

You may specify multiple `--op`s to preprocess the data feed into the
distribution scheme. One popular example is merging the parties *CDU* and *CSU*
to one party and see how this changes the results.
In this case there will be one table for each year per specified `op`.

> [!NOTE]
> If you have suggestions for useful perprocessing operations, feel free
> to reach out to me via a github issue.

## Contributing

You don't need to write code and open pull requests in order to contribute. You
found a bug? Nice, just drop me a message (preferably via a github issue). You
have an idea how to extend the tool? Great, same thing just drop me a message.

## References
> [!NOTE]
> I keep references to the origin of the external files I include to ease the
> whole setting things up part in a file with the same filename buf the
> extension replaced with `.url` (the file is a simple text file though)
