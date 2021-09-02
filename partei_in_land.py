from partei import Partei

class Partei_in_Land:
    def __init__(self, name, land):
        self.name = name
        self.land = land
        self.partei = Partei.get(name)
        self.partei.add(self)
        
        self.sitze = 0
        self.direktmandate = 0
        self.erststimmen = 0
        self.zweitstimmen = 0

        self.sitzkontingentVerteilung = 0
        self.mindestsitzzahl = 0

    def add_direktmandat(self):
        self.direktmandate += 1
        self.partei.direktmandate += 1

    def update_stimmen(self, erststimmen, zweitstimmen):
        self.erststimmen = erststimmen
        self.zweitstimmen = zweitstimmen
        self.partei.update_zweitstimmen(zweitstimmen)
    
    def __str__(self):
        return "{" + self.name + "}"
    def __repr__(self):
        return self.__str__()
