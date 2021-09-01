"""
One implementation of how seats can be distributed
"""
class Sainte_Lague_Schepers_utils:
    def __init__(self, zweitwahl, ges_sitze):
        self.zweitwahl = zweitwahl
        self.ges_sitze = ges_sitze
        self.verteilung = {}

    """
    calculate distribution
    """
    def apply(self):
        zutDivisor = sum([x for _,x in self.zweitwahl.items()]) / self.ges_sitze
        while (s := sum([x for _,x in self.verteilung.items()])) != self.ges_sitze: #TODO passen die steps so, oder kann man die irgendwie schlau berechnen (oder immer +-10% oder so) check ob osziliert -> steps kleiner machen (dann aber bei 100 anfangen)
            if s < self.ges_sitze:
                zutDivisor -= 1
            elif s > self.ges_sitze:
                zutDivisor += 1
            self.updateVert(zutDivisor)
        # print("Divisor:", zutDivisor, self.verteilung)
        return self.verteilung

    def updateVert(self, zutDivisor):
        for p, stimmen in self.zweitwahl.items():
            sitze = round(stimmen / zutDivisor) #TODO .5 -> würfeln nicht beachtet
            self.verteilung.update({p: sitze})
