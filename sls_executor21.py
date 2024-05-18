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

from sls_executor import Sainte_Lague_Schepers_executor # TODO vererbung
from sls_utils import Sainte_Lague_Schepers_utils
from decimal import Decimal, ROUND_HALF_UP
"""
Berechnung wie in 2021
"""
class Sainte_Lague_Schepers_executor21:
    @staticmethod
    def mindestsitzzahlen(parteien):
        for nB,pB in parteien.items():
            mindestsitzzahl = 0
            sitzkontingentVerteilung = 0
            for n,p in pB.partei_in_land.items():
                mean = int(( (Decimal(p.sitzkontingentVerteilung) + Decimal(p.direktmandate)) / 2 ).to_integral_value(rounding=ROUND_HALF_UP))
                p.mindestsitzzahl = int(max(mean, p.direktmandate))
                # print(nB, n, max(p.direktmandate - p.sitzkontingentVerteilung,0), p.mindestsitzzahl)

                mindestsitzzahl += p.mindestsitzzahl
                sitzkontingentVerteilung += p.sitzkontingentVerteilung

            pB.mindestsitzzahl = mindestsitzzahl
            pB.sitzkontingentVerteilung = sitzkontingentVerteilung
            pB.mindestsitzanspruch = max(pB.mindestsitzzahl, pB.sitzkontingentVerteilung)
            # print(pB.name, "Sitzkontingent:", pB.sitzkontingentVerteilung)

    @staticmethod
    def oberverteilung(parteien, ges_sitze):
        m = [p.mindestsitzanspruch for p in parteien.values()]
        ges_sitze = sum(m) - 1 # because loop firstly increments ges_sitze
        finished = False
        while not finished:
            ges_sitze+=1
            s = Sainte_Lague_Schepers_executor21.oberverteilung_update(parteien, ges_sitze)
            ueberhange = 0
            for p in parteien.values():
                p.ueberhang = max(p.mindestsitzanspruch - s.verteilung[p.name], 0)
                ueberhange += p.ueberhang
            if ueberhange <= 3:
                finished = True

        for p in parteien.values():
            s.verteilung[p.name] += p.ueberhang

        return s.verteilung, ges_sitze+ueberhange

    @staticmethod
    def oberverteilung_update(parteien, ges_sitze):
        # print("ges_sitze:", ges_sitze)
        s = Sainte_Lague_Schepers_utils({p.name: p.zweitstimmen for p in parteien.values()}, ges_sitze)
        s.apply()
        return s

    @staticmethod
    def apply(wahl):
        Sainte_Lague_Schepers_executor.sitzkontingent(wahl.bund.laender, 598)
        # print()
        Sainte_Lague_Schepers_executor.unterverteilung(wahl.bund.laender)
        # print()
        Sainte_Lague_Schepers_executor21.mindestsitzzahlen(wahl.bund.parteien)
        # print()
        return Sainte_Lague_Schepers_executor21.oberverteilung(wahl.bund.parteien, 598)
