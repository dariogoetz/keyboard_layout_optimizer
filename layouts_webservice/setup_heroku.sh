heroku git:remote -a keyboard-layout-optimizer
heroku buildpacks:set emk/rust --app keyboard-layout-optimizer
heroku config:set ROCKET_ALLOWED_CORS_ORIGINS=https://dariogoetz.github.io ROCKET_EVAL_PARAMETERS=config/evaluation_parameters.yml ROCKET_LAYOUT_CONFIGS='[["standard", "config/standard_keyboard.yml"], ["ortho", "config/ortho.yml"], ["moonlander", "config/moonlander.yml"], ["crkbd", "config/crkbd.yml"]]' ROCKET_NGRAMS=corpus/deu_mixed_wiki_web_0.6_eng_news_typical_wiki_web_0.4 ROCKET_STATIC_DIR=layouts_webservice/static ROCKET_SECRET=super_duper_secret --app keyboard-layout-optimizer
heroku addons:create heroku-postgresql:hobby-dev --app keyboard-layout-optimizer
