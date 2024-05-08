import styles from './Pancakes.module.scss'
import { useState } from 'react'

export default function Pancakes() {
  const [recipes, setRecipes] = useState<{ [key: string]: string }>({
    'Pancakes': `<ul class="ingredient-list">
    <li class="ingredient">1 cup all-purpose flour</li>
    <li class="ingredient">2 tablespoons sugar</li>
    <li class="ingredient">1 tablespoon baking powder</li>
    <li class="ingredient">1/2 teaspoon salt</li>
    <li class="ingredient">1 cup milk</li>
    <li class="ingredient">2 tablespoons unsalted butter, melted</li>
    <li class="ingredient">1 large egg</li>
    <li class="ingredient">1 teaspoon vanilla extract</li>
  </ul>

  <h2>Instructions:</h2>
  <ol class="instructions">
    <li>Preheat a non-stick skillet or griddle over medium heat.</li>
    <li>In a large bowl, whisk together the flour, sugar, baking powder, and salt.</li>
    <li>In another bowl, whisk together the milk, melted butter, egg, and vanilla extract.</li>
    <li>Pour the wet ingredients into the dry ingredients and stir until just combined. Be careful not to overmix; the batter should be slightly lumpy.</li>
    <li>Lightly grease the skillet or griddle with butter or cooking spray.</li>
    <li>For each pancake, pour about 1/4 cup of batter onto the skillet or griddle.</li>
    <li>Cook until bubbles form on the surface of the pancake and the edges look set, about 2-3 minutes.</li>
    <li>Flip the pancake and cook until golden brown on the other side, about 1-2 minutes more.</li>
    <li>Repeat with the remaining batter, greasing the skillet or griddle as needed.</li>
  </ol>`,
    'Blueberry pancakes': ` <ul class="ingredient-list">
    <li class="ingredient">1 cup all-purpose flour</li>
    <li class="ingredient">2 tablespoons sugar</li>
    <li class="ingredient">1 tablespoon baking powder</li>
    <li class="ingredient">1/2 teaspoon salt</li>
    <li class="ingredient">1 cup milk</li>
    <li class="ingredient">2 tablespoons unsalted butter, melted</li>
    <li class="ingredient">1 large egg</li>
    <li class="ingredient">1 teaspoon vanilla extract</li>
    <li class="ingredient">1 cup fresh blueberries</li>
  </ul>

  <h2>Instructions:</h2>
  <ol class="instructions">
    <li>Preheat a non-stick skillet or griddle over medium heat.</li>
    <li>In a large bowl, whisk together the flour, sugar, baking powder, and salt.</li>
    <li>In another bowl, whisk together the milk, melted butter, egg, and vanilla extract.</li>
    <li>Pour the wet ingredients into the dry ingredients and stir until just combined. Be careful not to overmix; the batter should be slightly lumpy.</li>
    <li>Gently fold in the fresh blueberries.</li>
    <li>Lightly grease the skillet or griddle with butter or cooking spray.</li>
    <li>For each pancake, pour about 1/4 cup of batter onto the skillet or griddle.</li>
    <li>Cook until bubbles form on the surface of the pancake and the edges look set, about 2-3 minutes.</li>
    <li>Flip the pancake and cook until golden brown on the other side, about 1-2 minutes more.</li>
    <li>Repeat with the remaining batter, greasing the skillet or griddle as needed.</li>
    <li>Serve warm with additional fresh blueberries and maple syrup.</li>
  </ol>
    `,
    'Chocolate pancakes': `<ul class="ingredient-list">
    <li class="ingredient">1 cup all-purpose flour</li>
    <li class="ingredient">2 tablespoons cocoa powder</li>
    <li class="ingredient">2 tablespoons sugar</li>
    <li class="ingredient">1 tablespoon baking powder</li>
    <li class="ingredient">1/2 teaspoon salt</li>
    <li class="ingredient">1 cup milk</li>
    <li class="ingredient">2 tablespoons unsalted butter, melted</li>
    <li class="ingredient">1 large egg</li>
    <li class="ingredient">1 teaspoon vanilla extract</li>
    <li class="ingredient">1/2 cup chocolate chips (optional)</li>
  </ul>

  <h2>Instructions:</h2>
  <ol class="instructions">
    <li>Preheat a non-stick skillet or griddle over medium heat.</li>
    <li>In a large bowl, whisk together the flour, cocoa powder, sugar, baking powder, and salt.</li>
    <li>In another bowl, whisk together the milk, melted butter, egg, and vanilla extract.</li>
    <li>Pour the wet ingredients into the dry ingredients and stir until just combined. Be careful not to overmix; the batter should be slightly lumpy.</li>
    <li>If using chocolate chips, gently fold them into the batter.</li>
    <li>Lightly grease the skillet or griddle with butter or cooking spray.</li>
    <li>For each pancake, pour about 1/4 cup of batter onto the skillet or griddle.</li>
    <li>Cook until bubbles form on the surface of the pancake and the edges look set, about 2-3 minutes.</li>
    <li>Flip the pancake and cook until golden brown on the other side, about 1-2 minutes more.</li>
    <li>Repeat with the remaining batter, greasing the skillet or griddle as needed.</li>
    <li>Serve warm with additional chocolate chips, whipped cream, or maple syrup.</li>
  </ol>`,
    'Banana pancakes': `<ul class="ingredient-list">
    <li class="ingredient">1 cup all-purpose flour</li>
    <li class="ingredient">1 tablespoon sugar</li>
    <li class="ingredient">1 tablespoon baking powder</li>
    <li class="ingredient">1/4 teaspoon salt</li>
    <li class="ingredient">1 cup milk</li>
    <li class="ingredient">2 ripe bananas, mashed</li>
    <li class="ingredient">2 tablespoons unsalted butter, melted</li>
    <li class="ingredient">1 large egg</li>
    <li class="ingredient">1 teaspoon vanilla extract</li>
  </ul>

  <h2>Instructions:</h2>
  <ol class="instructions">
    <li>Preheat a non-stick skillet or griddle over medium heat.</li>
    <li>In a large bowl, whisk together the flour, sugar, baking powder, and salt.</li>
    <li>In another bowl, combine the mashed bananas, milk, melted butter, egg, and vanilla extract.</li>
    <li>Pour the wet ingredients into the dry ingredients and stir until just combined. Be careful not to overmix; the batter should be slightly lumpy.</li>
    <li>Lightly grease the skillet or griddle with butter or cooking spray.</li>
    <li>For each pancake, pour about 1/4 cup of batter onto the skillet or griddle.</li>
    <li>Cook until bubbles form on the surface of the pancake and the edges look set, about 2-3 minutes.</li>
    <li>Flip the pancake and cook until golden brown on the other side, about 1-2 minutes more.</li>
    <li>Repeat with the remaining batter, greasing the skillet or griddle as needed.</li>
    <li>Serve warm with sliced bananas, maple syrup, or honey.</li>
  </ol>`
  })
  const [activeRecipe, setActiveRecipe] = useState('Blueberry pancakes')

  return (
    <div className="interactive">
      <div className={styles.wrapper}>
        <div className={styles.select}>
          {Object.keys(recipes).map(recipe => (
            <div key={recipe} onClick={() => setActiveRecipe(recipe)}>
              {recipe}
            </div>
          ))}
        </div>
        <div
          className={styles.recipe}
          dangerouslySetInnerHTML={{ __html: recipes[activeRecipe] }}></div>
      </div>
    </div>
  )
}
