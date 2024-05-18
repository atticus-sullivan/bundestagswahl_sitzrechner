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

from partei import Partei
import land as Land

class Partei_in_Land:
    def __init__(self, name, land):
        self.name = name
        self.land = land
        self.partei = Partei.get(name)
        self.partei.add(self)
        
        self.direktmandate = 0
        self.erststimmen = None #TODO not nessacary
        self.zweitstimmen = None

        self.sitzkontingentVerteilung = None
        self.mindestsitzzahl = None

    def __str__(self):
        return "[Partei_in_Land] name:" + str(self.name) + "land:" + str(self.land.name) + ", partei:" + str(self.partei.name) + ", direktmandate:" + str(self.direktmandate) + ", zweitstimmen:" + str(self.zweitstimmen) + ", sitzkontingentVerteilung:" + str(self.sitzkontingentVerteilung) + ", mindestsitzzahl:" + str(self.mindestsitzzahl) + "}"
    def __repr__(self):
        return str(self)

    @property
    def name(self):
        return self._name
    @name.setter
    def name(self, name):
        if isinstance(name, str):
            self._name = name
        else:
            raise AttributeError("name has to be a string, but was", type(name))
    @property
    def land(self):
        return self._land
    @land.setter
    def land(self, land):
        if isinstance(land, Land.Land):
            self._land = land
        else:
            raise AttributeError("land has to be a Land, but was", type(land))
    @property
    def partei(self):
        return self._partei
    @partei.setter
    def partei(self, partei):
        if isinstance(partei, Partei):
            self._partei = partei
        else:
            raise AttributeError("partei has to be a Partei, but was", type(partei))
    @property
    def direktmandate(self):
        return self._direktmandate
    @direktmandate.setter
    def direktmandate(self, direktmandate):
        if isinstance(direktmandate, int):
            self._direktmandate = direktmandate
        else:
            raise AttributeError("direktmandate has to be an int, but was", type(direktmandate))
    @property
    def zweitstimmen(self):
        return self._zweitstimmen
    @zweitstimmen.setter
    def zweitstimmen(self, zweitstimmen):
        if isinstance(zweitstimmen, int) or zweitstimmen is None:
            self._zweitstimmen = zweitstimmen
        else:
            raise AttributeError("zweitstimmen has to be an int, but was", type(zweitstimmen))
    @property
    def sitzkontingentVerteilung(self):
        return self._sitzkontingentVerteilung
    @sitzkontingentVerteilung.setter
    def sitzkontingentVerteilung(self, sitzkontingentVerteilung):
        if isinstance(sitzkontingentVerteilung, int) or sitzkontingentVerteilung is None:
            self._sitzkontingentVerteilung = sitzkontingentVerteilung
        else:
            raise AttributeError("sitzkontingentVerteilung has to be an int, but was", type(sitzkontingentVerteilung))
    @property
    def mindestsitzzahl(self):
        return self._mindestsitzzahl
    @mindestsitzzahl.setter
    def mindestsitzzahl(self, mindestsitzzahl):
        if isinstance(mindestsitzzahl, int) or mindestsitzzahl is None:
            self._mindestsitzzahl = mindestsitzzahl
        else:
            raise AttributeError("mindestsitzzahl has to be an int, but was", type(mindestsitzzahl))

    def add_direktmandat(self):
        self.direktmandate += 1
        self.partei.direktmandate += 1

    def update_stimmen(self, erststimmen, zweitstimmen):
        self.erststimmen = erststimmen
        self.zweitstimmen = zweitstimmen
        self.partei.update_zweitstimmen(zweitstimmen)
