#!/bin/env python3

from wahl import Wahl
from sls_executor17 import Sainte_Lague_Schepers_executor17
from sls_executor21 import Sainte_Lague_Schepers_executor21
from banzhaff import Abstimmung

### TODO create all variables in contstructors and print them on __str__ / __repr__
## TODO use decimal for correct rounding and precise calculations

if __name__ == "__main__":
    w = Wahl(2017)
    w.load_from_csv("btw17_kerg.csv")
    w.load_bef("btw17_bef.csv")

    w.remove_below_huerde()

    vert,sitze = Sainte_Lague_Schepers_executor17.apply(wahl=w)
    # vert,sitze = Sainte_Lague_Schepers_executor21.apply(wahl=w)

    # print()
    print("Sitze:", sitze)
    print(vert)

    Abstimmung.from_dict(vert).simulieren()

    print()
    print()
    vert,sitze = Sainte_Lague_Schepers_executor21.apply(wahl=w)
    # print()
    print("Sitze:", sitze)
    print(vert)

    Abstimmung.from_dict(vert).simulieren()
