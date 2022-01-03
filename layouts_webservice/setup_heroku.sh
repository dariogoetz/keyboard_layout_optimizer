heroku git:remote -a keyboard-layout-optimizer
heroku buildpacks:set emk/rust --app keyboard-layout-optimizer
heroku config:set ROCKET_ALLOWED_CORS_ORIGINS=https://dariogoetz.github.io ROCKET_EVAL_PARAMETERS=config/evaluation_parameters.yml ROCKET_LAYOUT_CONFIG=config/standard_keyboard.yml ROCKET_NGRAMS=corpus/arne_no_special ROCKET_STATIC_DIR=layouts_webservice/static ROCKET_SECRET=super_duper_secret --app keyboard-layout-optimizer
heroku addons:create heroku-postgresql:hobby-dev --app keyboard-layout-optimizer
