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
Berechnung wie in 2017
"""
class Sainte_Lague_Schepers_executor17:
    @staticmethod
    def mindestsitzzahlen(parteien):
        for nB,pB in parteien.items():
            mindestsitzzahl = 0
            for n,p in pB.partei_in_land.items():
                p.mindestsitzzahl = int(max(p.sitzkontingentVerteilung, p.direktmandate))
                # print(nB, n, p.mindestsitzzahl)

                mindestsitzzahl += p.mindestsitzzahl

            pB.mindestsitzzahl = mindestsitzzahl

    @staticmethod
    def oberverteilung(parteien, ges_sitze):
        div = 1000000000000000000000000 # TODO max
        for n,p in parteien.items():
            # print(p.zweitstimmen, "/", p.mindestsitzzahl, "-", 0.5)
            div = min(div, Decimal(p.zweitstimmen) / (p.mindestsitzzahl - Decimal('0.5')))
        # print(div)
        for n,p in parteien.items():
            p.sitze = int((Decimal(p.zweitstimmen) / div).to_integral_value(rounding=ROUND_HALF_UP))
            # print(n, sitze)

        return {p.name: p.sitze for p in parteien.values()}, sum([p.sitze for p in parteien.values()])

    @staticmethod
    def apply(wahl):
        Sainte_Lague_Schepers_executor.sitzkontingent(wahl.bund.laender, 598)
        # print()
        Sainte_Lague_Schepers_executor.unterverteilung(wahl.bund.laender)
        # print()
        Sainte_Lague_Schepers_executor17.mindestsitzzahlen(wahl.bund.parteien)
        # print()
        # print()
        return Sainte_Lague_Schepers_executor17.oberverteilung(wahl.bund.parteien, 598)
