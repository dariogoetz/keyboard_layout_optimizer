heroku git:remote -a keyboard-layout-optimizer
heroku buildpacks:set emk/rust --app keyboard-layout-optimizer
heroku config:set ROCKET_ALLOWED_CORS_ORIGINS=https://dariogoetz.github.io ROCKET_EVAL_PARAMETERS=config/evaluation/default.yml ROCKET_LAYOUT_CONFIGS='[["standard", "config/keyboard/standard.yml"], ["ortho", "config/keyboard/ortho.yml"], ["moonlander", "config/keyboard/moonlander.yml"], ["crkbd", "config/keyboard/crkbd.yml"]]' ROCKET_DEFAULT_LAYOUT_CONFIG=standard ROCKET_NGRAMS=ngrams/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4 ROCKET_STATIC_DIR=webui/layouts_webservice/static ROCKET_SECRET=super_duper_secret --app keyboard-layout-optimizer
heroku addons:create heroku-postgresql:hobby-dev --app keyboard-layout-optimizer
