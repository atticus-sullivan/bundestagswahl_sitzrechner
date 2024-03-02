#!/bin/env python3

from wahl import Wahl
from banzhaff import Abstimmung

from terminaltables import SingleTable

def print_vert(vert, sitze, b, wahlrecht):
    tab = [[p, s, round(s/sitze*100,2), round(b[p],3)] for p,s in vert.items()]
    tab.sort(key=lambda x: x[0])
    t = SingleTable([["Partei", "Sitze", "Prozent", "Banzhaff"]] + tab, title="Sitze: " + str(sitze) + " nach Wahlrecht von " + str(wahlrecht))
    print(t.table)

if __name__ == "__main__":
    for y,f1, f2 in [
            ("2017", "btw17_kerg.csv", "btw17_bef.csv"),
            ("2021", "btw21_kerg-vorl.csv", "btw17_bef.csv"),
            ("2021-cxu", "btw21_kerg-vorl-cxu.csv", "btw17_bef.csv"),
            ]:
        for wahlrecht in [
                2017,
                2021,
                ]:
            print(y)
            w = Wahl(wahlrecht)
            w.load_from_csv(f1)
            w.load_bef(f2)
            vert,sitze = w.calc_sitze()

            banzhaf = Abstimmung.from_dict(vert).simulieren()

            print_vert(vert, sitze, banzhaf, wahlrecht)

            print("\n")

# if __name__ == "__main__":
#     print("2021")
#     w = Wahl(2017)
#     w.load_from_csv("btw21_kerg_vorl.csv")
#     w.load_bef("btw17_bef.csv")
#     vert,sitze = w.calc_sitze()

#     banzhaf = Abstimmung.from_dict(vert).simulieren()

#     print_vert(vert, sitze, banzhaf, 2017)

#     print()
#     print()
#     w = Wahl(2021)
#     w.load_from_csv("btw21_kerg_vorl.csv")
#     w.load_bef("btw17_bef.csv")
#     vert,sitze = w.calc_sitze()

#     banzhaf = Abstimmung.from_dict(vert).simulieren()

#     print_vert(vert, sitze, banzhaf, 2021)
