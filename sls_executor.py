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
