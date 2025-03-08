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

### Different SCHEMES
You can select to calculate the distribution of the seats with different schemes
(e.g. legislation in place for a certain election).

Note the term *SLS* is used quite often in the description of different
schemes. *SLS* is short for *Sainte-Laguë/Schepers* which is a method to
distribute e.g. seats proportional to some other distribution (e.g. votes)

Also note that I won't give references to the legislative texts in the
descriptions here. Instead the comments in the source code contains such
references.

#### 2021
This is the scheme which was used for the election of the Bundestag in 2021. It
consists of multiple steps:
1. Base Seats (598) get reduced by the number of directly elected independant
   candidates (not considered here yet, but usually this case does not happen ->
   doesn't really matter most of the time).
2. Neglect some Zweitstimmen under some circumstances (not considered here
   because with the current data from Bundeswahlleiter:in this is not possible)
3. Neglect all Zweitstimmen for Parteien which got `<5%` of the Zweitstimmen and
   didn't win `>=3` Wahlkreise (Special case are Parteien of national
   minorities).
4. Distribute the base seats (via SLS) among the Bundesländer according to the
   population (Sitzkontingent)
5. For each Bundesland, distribute the seats (via SLS) of the Sitzkontingent
   according to the Zweitstimmen among the Parteien (Unterverteilung)
6. Calculate how many seats each Partei should get at least based on
   Unterverteilung and Direktmandate (Mindestsitzzahl)
7. Increase the total number of seats (and redistribute them with SLS) until
   each Partei gets at least Mindestsitzzahl many seats (three seats are allowed
   to be missing) (Oberverteilung)
8. Continue figuring out who exactly gets into the parliament (not of interest
   here)

#### 2025
This is the scheme which was used for the election of the Bundestag in 2025. It
is basically a pure Verhältniswahlrecht for most part and thus much simpler than
the scheme of 2021. These are the steps:
1. Base Seats (630) get reduced by the number of directly elected independant
   candidates (not considered here yet, but usually this case does not happen ->
   doesn't really matter most of the time).
2. Neglect some Zweitstimmen under some circumstances (not considered here
   because with the current data from Bundeswahlleiter:in this is not possible)
3. Neglect all Zweitstimmen for Parteien which got `<5%` of the Zweitstimmen and
   didn't win `>=3` Wahlkreise (Special case are Parteien of national
   minorities).
4. Distribute the available seats based on the Zweitstimmen among the Parteien
   (via SLS).
5. Continue figuring out who exactly gets into the parliament, which includes
   the so-called Zweitstimmendeckung (not of interest
   here)

#### mehrheit
This wasn't used so far in Germany. It is an implementation of a pure
Mehrheitswahlrecht not too different from the US. There is only the Erststimme
and each person winning a Wahlkreis gets into the Parliament. Due to the
Wahlkreise being constructed so that the population is roughly the same, there
should be no need of weighting the winners of a Wahlkreis with the contained
population.

### Different CALC_OPS
The purpose of this option is to allow some kind or preprocessing before handing
the data to the distribution scheme.

#### none
This is the default and just a dummy value which does *no preprocessing*.

#### merge-cdu-csu
This merges the Erst- and Zweitstimmen of the Parteien *CDU* and *CSU* which
traditionally form a Fraktion together in the parliamant anyhow in the end.

The idea here is to investiage what impact it would have if this special status
of the *CSU* would be removed (no other Bundestland and no other Partei has
this).

## Contributing
You don't need to write code and open pull requests in order to contribute. You
found a bug? Nice, just drop me a message (preferably via a github issue). You
have an idea how to extend the tool? Great, same thing just drop me a message.

## References
> [!NOTE]
> I keep references to the origin of the external files I include to ease the
> whole setting things up part in a file with the same filename buf the
> extension replaced with `.url` (the file is a simple text file though)
