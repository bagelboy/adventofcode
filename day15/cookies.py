#!/usr/bin/env python3


class Ingredient:

    def __init__(self, name, attributes):
        self.name = name
        self.capacity = attributes['capacity']
        self.durability = attributes['durability']
        self.flavor = attributes['flavor']
        self.texture = attributes['texture']
        self.calories = attributes['calories']


def calculate_score(ingredients, amounts):
    total_capacity = 0
    total_durability = 0
    total_flavor = 0
    total_texture = 0

    for i in ingredients:
        total_capacity += i.capacity * amounts[i.name]
        total_durability += i.durability * amounts[i.name]
        total_flavor += i.flavor * amounts[i.name]
        total_texture += i.texture * amounts[i.name]

    product = 1
    for attr in [total_capacity, total_durability, total_flavor, total_texture]:
        product *= max(0, attr)
    return product


def calculate_calories(ingredients, recipe):
    calories = 0
    for i in ingredients:
        calories += i.calories * recipe[i.name]
    return calories


def parse_ingredient(line):
    name, ingredients = line.split(sep=None, maxsplit=1)
    attr = ingredients.split(', ')
    attributes = {}
    for pair in attr:
        parts = pair.split()
        attributes[parts[0]] = int(parts[1])
    return Ingredient(name[0:-1], attributes)


def run_tests():
    ingredients = [parse_ingredient(i) for i in [
        'Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8',
        'Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3'
    ]]
    assert(calculate_score(ingredients, {'Butterscotch': 44,
                                         'Cinnamon': 56}) == 62842880)

def main():
    run_tests()

    with open('input') as raw_data:
        ingredients = [parse_ingredient(line.rstrip()) for line in raw_data]

    assert len(ingredients) == 4

    ingredient_names = tuple([i.name for i in ingredients])

    best_overall_score = -1
    best_overall_recipe = None
    best_500_calorie_score = -1
    best_500_calorie_recipe = None

    for first in range(101):
        for second in range(101 - first):
            for third in range(101 - (first + second)):
                fourth = 100 - (first + second + third)
                amounts = (first, second, third, fourth)
                if sum(amounts) == 100:
                    recipe = dict(zip(ingredient_names, amounts))
                    score = calculate_score(ingredients, recipe)

                    # find overall best recipe
                    if score > best_overall_score:
                        best_overall_score = score
                        best_overall_recipe = recipe

                    # find best recipe with exactly 500 calories
                    calories = calculate_calories(ingredients, recipe)
                    if calories == 500 and score > best_500_calorie_score:
                        best_500_calorie_recipe = recipe
                        best_500_calorie_score = score

    print('{} ({})'.format(best_overall_score, ', '.join(
        ['{} tsp {}'.format(value, key) for key, value in best_overall_recipe.items()])))
    print('{} ({})'.format(best_500_calorie_score, ', '.join(['{} tsp {}'.format(
        value, key) for key, value in best_500_calorie_recipe.items()])))

if __name__ == '__main__':
    main()
