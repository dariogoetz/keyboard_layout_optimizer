import pandas as pd
import matplotlib.pyplot as plt

import subprocess
import json

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("layouts", nargs="+", help="List of layouts to compare")
    parser.add_argument(
        "--corpora",
        default=[
            "ngrams/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4",
            "ngrams/deu_wiki_0.6_eng_wiki_0.4",
            "ngrams/deu_mixed_0.6_eng_news_typical_0.4",
            "ngrams/deu_web_1m",
            "ngrams/eng_web_1m",
            "ngrams/deu_wiki_1m",
            "ngrams/eng_wiki_1m",
            "ngrams/deu_mixed_1m",
            "ngrams/eng_news_typical_1m",
            "ngrams/arne",
            "ngrams/irc_neo",
        ],
        help="List of ngrams directories to compare",
    )
    parser.add_argument(
        "--out", default="layout_by_corpus.png", help="Filename of resulting image"
    )
    parser.add_argument(
        "--eval-params", default="", help="Arguments to pass to the evaluator"
    )

    args = parser.parse_args()

    layouts = args.layouts
    corpora = args.corpora

    dfs = []
    for corpus in corpora:
        cmd = [
            "cargo",
            "run",
            "--release",
            "--",
            *layouts,
            "--ngrams",
            corpus,
            "--json",
        ]
        if args.eval_params:
            cmd.extend(args.eval_params.split(" "))
        res = subprocess.check_output(cmd)
        res = json.loads(res.decode("utf8"))

        total_costs = [d["total_cost"] for d in res]

        df = pd.DataFrame(
            {"total_cost": total_costs, "corpus": corpus, "layout": layouts}
        )
        df.index.name = "layout"

        dfs.append(df)

    df = pd.concat(dfs)
    df = (
        df.set_index(["layout", "corpus"])
        .unstack()["total_cost"]
        .reindex(layouts)
        .T.reindex(corpora)
    )
    df.index = df.index.map(lambda c: c.lstrip("ngrams/"))

    df.plot.bar()
    plt.gcf().set_size_inches((16, 9))
    plt.xticks(rotation=0)
    plt.savefig(args.out)

    # import ipdb; ipdb.set_trace()
