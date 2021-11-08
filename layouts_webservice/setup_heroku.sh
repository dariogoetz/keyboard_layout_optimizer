heroku git:remote -a keyboard-layout-optimizer
heroku buildpacks:set emk/rust --app keyboard-layout-optimizer 
heroku config:set ROCKET_EVAL_PARAMETERS=config/evaluation_parameters.yml ROCKET_LAYOUT_CONFIG=config/standard_keyboard.yml ROCKET_UNIGRAMS=1-gramme.arne.txt ROCKET_BIGRAMS=2-gramme.arne.txt ROCKET_TRIGRAMS=3-gramme.arne.txt ROCKET_STATIC_DIR=layouts_webservice/static ROCKET_REEVAL_LAYOUTS=false ROCKET_SECRET=super_duper_secret --app keyboard-layout-optimizer
heroku addons:create heroku-postgresql:hobby-dev --app keyboard-layout-optimizer
