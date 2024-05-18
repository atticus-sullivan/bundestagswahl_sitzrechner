#!/bin/env python3

# Copyright (c) 2024, Lukas Heindl
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# 3. Neither the name of the copyright holder nor the names of its
#    contributors may be used to endorse or promote products derived from
#    this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
# AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
# IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
# FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
# DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
# SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
# CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
# OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
