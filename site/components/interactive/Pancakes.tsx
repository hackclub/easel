import styles from './Pancakes.module.scss'
import { useState } from 'react'

export default function Pancakes() {
  const [recipes, setRecipes] = useState<{ [key: string]: string }>({
    'Pancakes': ``,
    'Blueberry pancakes': `
Blueberry pancakes

<ol>
  <li>Make</li>
</ol>
    `,
    'Chocolate pancakes': ''
  })
  const [activeRecipe, setActiveRecipe] = useState('Blueberry pancakes')

  return (
    <div className="interactive">
      <div className={styles.wrapper}>
        <div className={styles.select}>
          {Object.keys(recipes).map(recipe => (
            <div key={recipe}>{recipe}</div>
          ))}
        </div>
        <div
          className={styles.recipe}
          dangerouslySetInnerHTML={{ __html: recipes[activeRecipe] }}></div>
      </div>
    </div>
  )
}
