from sls_utils import Sainte_Lague_Schepers_utils
"""
gemeinsame Berechnungen
"""
class Sainte_Lague_Schepers_executor: # TODO abstract
    @staticmethod
    def sitzkontingent(laender, ges_size):
        s = Sainte_Lague_Schepers_utils({l.name: l.ewz for l in laender.values()}, ges_size)
        s.apply()

        for n,l in laender.items():
            l.sitzkontingent = s.verteilung[n]
        # print(laender)
    @staticmethod
    def unterverteilung(laender):
        for n,l in laender.items():
            s = Sainte_Lague_Schepers_utils({p.name: p.zweitstimmen for p in l.parteien.values()}, l.sitzkontingent)
            s.apply()
            for n,p in l.parteien.items():
                p.sitzkontingentVerteilung = s.verteilung[n]
    @staticmethod
    def mindestsitzzahlen(parteien): #TODO vererbung
        pass
    @staticmethod
    def oberverteilung(parteien, ges_sitze): #TODO vererbung
        pass
