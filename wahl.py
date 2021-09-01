from bund import Bund
from land import Land
from partei import Partei
import csv

"""
Representation of an election
"""
class Wahl:
    def __init__(self, year):
        self.year = year
        self.bund = Bund()
        self.verteilungen = {} # Land -> Sainte_Lague_Schepers

    def remove_below_huerde(self):
        self.bund.calc_percentageHuerde()
        self.bund.huerde_remove()

    """
    load election results from file fn
    """
    def load_from_csv(self, fn):
        with open(fn) as csv_file:
            csv_reader = list(csv.reader(csv_file, delimiter=';'))
            hdr = []
            started = False
            votes = {}
            direkt_tmp = []
            for j,row in enumerate(csv_reader):
                if row[0] == "Nr" and not started:
                    hdr = row
                    for h in hdr[19:]:
                        if h.replace(" ", "") != "":
                            self.bund.add_partei(Partei.get(h))
                if row[0] == "1" and not started:
                    started = True

                if started:
                    if len(csv_reader) == j+1: continue # bund
                    if row == ['', '']: continue # leere Zeile


                    for i,_ in enumerate(row[19::4]):
                        if len(row) < 2+i*4+19: break # not enough data left
                        row[i*4+19] = row[i*4+19].replace(" ", "")
                        row[2+i*4+19] = row[2+i*4+19].replace(" ", "")

                        votes[hdr[i*4+19]] = (
                                0 if row[i*4+19] == "" else int(row[i*4+19]),
                                0 if row[2+i*4+19] == "" else int(row[2+i*4+19])
                                )

                    if csv_reader[j+1] == ['', '']:
                        # bundesland
                        land = Land(row[1], row[0])
                        for p,st in votes.items():
                            land.put_partei(p,st[0],st[1])
                        votes = {}
                        for d in direkt_tmp:
                            land.parteien[d].add_direktmandat()
                        direkt_tmp = []
                        self.bund.add_land(land)
                    else:
                        # wahlkreis
                        direkt_tmp.append(max(votes, key=lambda k: votes[k][0]))

    def load_bef(self, fn):
        with open(fn) as csv_file:
            csv_reader = csv.DictReader(csv_file, delimiter=';')
            for row in csv_reader:
                for k,v in row.items():
                    self.bund.laender[str(k)].ewz = int(v)
