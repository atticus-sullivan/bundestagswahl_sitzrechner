#!/bin/env python3

from wahl import Wahl
from banzhaff import Abstimmung

if __name__ == "__main__":
    w = Wahl(2017)
    w.load_from_csv("btw17_kerg.csv")
    w.load_bef("btw17_bef.csv")

    vert,sitze = w.calc_sitze()

    print("Sitze:", sitze)
    print(vert)

    Abstimmung.from_dict(vert).simulieren()

    print()
    print()
    w = Wahl(2021)
    w.load_from_csv("btw17_kerg.csv")
    w.load_bef("btw17_bef.csv")
    vert,sitze = w.calc_sitze()
    print("Sitze:", sitze)
    print(vert)

    Abstimmung.from_dict(vert).simulieren()
