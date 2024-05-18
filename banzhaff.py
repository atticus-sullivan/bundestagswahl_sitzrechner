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

import itertools
import math

class Fraktion:
    def __init__(self, name, stimmen):
        self.name = name
        self.stimmen = stimmen
        self.banzhaf_macht = 0
        self.banzhaf_index = 0

    def get_name(self):
        return self.name
    def get_stimmen(self):
        return self.stimmen
    def get_banzhaf_macht(self):
        return self.banzhaf_macht
    def get_banzhaf_index(self):
        return self.banzhaf_index

    def ist_kritisch(self):
        self.banzhaf_macht += 1

    def berechne_banzhaf_index(self, n):
        self.banzhaf_index = self.banzhaf_macht / n

class Koalition:
    def __init__(self, fraktionen):
        self.fraktionen = fraktionen
        self.stimmen = sum([f.get_stimmen() for f in self.fraktionen])

    def get_stimmen(self):
        return self.stimmen
    def get_fraktionen(self):
        return self.fraktionen
    
    def kritische_fraktionen(self, quorum):
        for fraktion in self.fraktionen:
            if self.get_stimmen() - fraktion.get_stimmen() < quorum:
                fraktion.ist_kritisch()

    def ist_gewinnend(self, quorum):
        return self.stimmen >= quorum

class Abstimmung:
    def __init__(self, quorum, *fraktionen):
        self.fraktionen = fraktionen
        self.quorum = quorum
        self.koalitionen = []
        self.gewinnend = []
        potenzmenge = [
                x
                for length in range(len(self.fraktionen) + 1)
                for x in itertools.combinations(self.fraktionen, length)
            ]
        for fraktion in potenzmenge:
            self.koalitionen.append(Koalition(list(fraktion)))
        self.gewinnende_koalitionen()

    def gewinnende_koalitionen(self):
        for koalition in self.koalitionen:
            if koalition.ist_gewinnend(self.quorum):
                self.gewinnend.append(koalition)

    def kritische_fraktionen(self):
        for koalition in self.gewinnend:
            koalition.kritische_fraktionen(self.quorum)

    def berechne_banzhaf_indizes(self):
        n = sum([f.get_banzhaf_macht() for f in self.fraktionen])
        for fraktion in self.fraktionen:
            fraktion.berechne_banzhaf_index(n)

    def simulieren(self):
        self.kritische_fraktionen()
        self.berechne_banzhaf_indizes()

        ret = {}
        for fraktion in self.fraktionen:
            ret[fraktion.get_name()] = fraktion.get_banzhaf_index()
            # print(fraktion.get_name() + " - " + str(fraktion.get_banzhaf_index()))
        return ret
    
    """
    parteien: dict[name:str -> sitze:int]
    half: True bei halbmehrheit, False->dreiviertel Mehrheit
    """
    @classmethod
    def from_dict(cls, parteien, half=True):
        fraktionen = []
        ges_sitze = 0
        for name,sitze in parteien.items():
            fraktionen.append(Fraktion(name, sitze))
            ges_sitze += sitze

        if half:
            quorum = math.ceil(ges_sitze / 2)
        else:
            quorum = math.ceil((ges_sitze * 2) / 3)

        return Abstimmung(quorum, *fraktionen)


if __name__ == "__main__":
    cdu = Fraktion("CDU", 200)
    spd = Fraktion("SPD", 153)
    afd = Fraktion("AFD", 94)
    fdp = Fraktion("FDP", 80)
    linke = Fraktion("Die Linke", 69)
    gruene = Fraktion("Grüne", 67)
    csu = Fraktion("CSU", 46)

    abstimmung = Abstimmung(355, cdu, spd, afd, fdp, linke, gruene, csu)
    abstimmung.simulieren()

    print()

    d = {"CDU": 200, "SPD": 153, "AFD": 94, "FDP": 80, "Linke": 69, "Grüne": 67, "CSU": 46}
    a = Abstimmung.from_dict(d, True)
    a.simulieren()
