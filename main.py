#!/bin/env python3

from wahl import Wahl
from banzhaff import Abstimmung

from terminaltables import SingleTable

def print_vert(vert, sitze, b, wahlrecht):
    t = SingleTable([["Partei", "Sitze", "Banzhaff"]] + [[p,s,b[p]] for p,s in vert.items()], title="Sitze: " + str(sitze) + " nach Wahlrecht von " + str(wahlrecht))
    print(t.table)

if __name__ == "__main__":
    for y,f1, f2 in [(2017, "btw17_kerg.csv", "btw17_bef.csv"), (2021, "btw21_kerg_vorl.csv", "btw17_bef.csv")]:
        for wahlrecht in [2017, 2021]:
            print(str(y))
            w = Wahl(wahlrecht)
            w.load_from_csv(f1)
            w.load_bef(f2)
            vert,sitze = w.calc_sitze()

            banzhaf = Abstimmung.from_dict(vert).simulieren()

            print_vert(vert, sitze, banzhaf, wahlrecht)

            print("\n")
