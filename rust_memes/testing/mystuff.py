from eight import register, Collector
from eight.collecting import submit


register(CollectData)
# Start with registering C lib, then later can Magic it up


def collect_data():
    pass

class Reddit(Collector):
    def run():
        while True:
            submit({"my": "data"})
