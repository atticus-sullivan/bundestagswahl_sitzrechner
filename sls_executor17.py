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
                mean = ((Decimal(p.sitzkontingentVerteilung) + Decimal(p.direktmandate))/2).to_integral_value(rounding=ROUND_HALF_UP)
                p.mindestsitzzahl = int(max(p.sitzkontingentVerteilung, p.direktmandate)) #TODO ohne mean und mit dieser oberverteilung ist vor wahlrechtsreform aus 2021/20
                # print(nB, n, p.mindestsitzzahl)

                mindestsitzzahl += p.mindestsitzzahl

            pB.mindestsitzzahl = mindestsitzzahl

    @staticmethod
    def oberverteilung(parteien, ges_sitze):
        div = 1000000000000000000000000 # TODO max
        for n,p in parteien.items():
            # print(p.zweitstimmen, "/", p.mindestsitzzahl, "-", 0.5)
            div = min(div, p.zweitstimmen / (p.mindestsitzzahl - 0.5))
        # print(div)
        for n,p in parteien.items():
            p.sitze = round(p.zweitstimmen / div)
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
