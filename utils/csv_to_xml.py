# SPDX-FileCopyrightText: 2025 Lukas Heindl
#
# SPDX-License-Identifier: MIT

import csv
import xml.etree.ElementTree as ET
import argparse

translate_name = {
    "Alternative für Deutschland": "AfD",
    "BÜNDNIS 90/DIE GRÜNEN": "GRÜNE",
    "Christlich Demokratische Union Deutschlands": "CDU",
    "Christlich-Soziale Union in Bayern e.V.": "CSU",
    "DIE LINKE": "Die Linke",
    "FREIE WÄHLER": "FREIE WÄHLER",
    "Freie Demokratische Partei": "FDP",
    "PARTEI MENSCH UMWELT TIERSCHUTZ": "Tierschutzpartei",
    "Partei für Arbeit, Rechtsstaat, Tierschutz, Elitenförderung und basisdemokratische Initiative": "Die PARTEI",
    "Sozialdemokratische Partei Deutschlands": "SPD",
}

"""
load election results from file fn
"""
def load_from_csv(fn):
    parteien = {}
    parteien_id_max = 0

    sub_gebiet = 0
    if fn.endswith("2013_gesamtergebnis.csv"):
        sub_gebiet = 900

    with open(fn) as csv_file:
        csv_reader = list(csv.reader(csv_file, delimiter=';'))
        hdr = []
        # parser can be in two states -> 1: parse the hdr lines 2: parse the data
        started = False
        contents = []
        for j,row in enumerate(csv_reader):
            if len(row) == 0: continue
            if row[0] == "Nr" and not started:
                hdr = row
                # for h in hdr[19:]:
                    # if h.replace(" ", "") != "":
                    #     self.bund.add_partei(Partei.get(h))
            if row[0] == "1" and not started:
                started = True

            if started:
                if all(map(lambda x: x == '', row)): continue # leere Zeile

                votes = {}
                for i,_ in enumerate(row[19::4]):
                    if len(row) < 2+i*4+19: break # not enough data left
                    row[i*4+19] = row[i*4+19].replace(" ", "")
                    row[2+i*4+19] = row[2+i*4+19].replace(" ", "")

                    if hdr[i*4+19] in votes:
                        tup = votes[hdr[i*4+19]]
                        # print("Warning: Partei", hdr[i*4+19], "tritt merhfach auf (in " + str(row[1]) + ")")
                    else:
                        tup = (0,0)

                    votes[hdr[i*4+19]] = (
                            0 + tup[0] if row[i*4+19] == "" else int(row[i*4+19]) + tup[0],
                            0 + tup[1] if row[2+i*4+19] == "" else int(row[2+i*4+19]) + tup[1]
                            )

                gebiet = {"name":row[1], "id":int(row[0]), "parent":row[2], "results":[]}
                for p,st in votes.items():
                    if p not in parteien:
                        parteien[p] = parteien_id_max
                        parteien_id_max += 1
                    gebiet["results"].append({"name":p, "parteiID":parteien[p], "direkt":st[0], "liste":st[1]})

                if len(csv_reader) == j+1:
                    gebiet["type"] = "BUND"
                    gebiet["ueg_type"] = None
                    gebiet["id"] -= sub_gebiet
                elif all(map(lambda x: x == '', csv_reader[j+1])):
                    gebiet["type"] = "LAND"
                    gebiet["ueg_type"] = "BUND"
                    gebiet["id"] -= sub_gebiet
                else:
                    gebiet["type"] = "WAHLKREIS"
                    gebiet["ueg_type"] = "LAND"
                contents.append(gebiet)

        return contents

def write_to_xml(data):
    root = ET.Element("Gesamtergebnis", {
    })

    for d in data:
        # Create the element
        ele = ET.SubElement(root, "Gebietsergebnis", {
            "Gebietsnummer": str(d["id"]),
            "Gebietsart": d["type"],
        })
        if d["ueg_type"]:
            ele.set("UegGebietsnummer", d["parent"])
            ele.set("UegGebietsart", d["ueg_type"])

        # Add the Gebietsname
        gebiet_text = ET.SubElement(ele, "GebietText")
        gebiet_text.text = d['name']

        for i in d["results"]:
            gruppenergebnis = ET.SubElement(ele, "Gruppenergebnis", {
                "Gruppe": str(i["parteiID"]),
                "Gruppenart": "PARTEI",
                "Name": translate_name.get(i["name"], i["name"])
            })
            stimmergebnis_direkt = ET.SubElement(gruppenergebnis, "Stimmergebnis", {
                "Stimmart": "DIREKT",
                "Anzahl": str(i["direkt"]),
            })

            stimmergebnis_liste = ET.SubElement(gruppenergebnis, "Stimmergebnis", {
                "Stimmart": "LISTE",
                "Anzahl": str(i["liste"]),
            })

    return ET.ElementTree(root)
    # tree.write(sys.stdout, encoding="utf-8", xml_declaration=True)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input")
    parser.add_argument("output")
    args = parser.parse_args()

    data = load_from_csv(args.input)
    xml = write_to_xml(data)
    
    with open(args.output, "wb") as f:
        ET.indent(xml)
        xml.write(f, encoding="utf-8", xml_declaration=True)
