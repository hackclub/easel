import styles from './LexerParserTransform.module.scss'

export default function LexerParserTransform() {
  return (
    <div className="interactive">
      <div className={styles.wrapper}>
        <div className={styles.left}>
          <div>
            <pre>{`{
  "type": "Keyword",
  "value": "prepare",
  "content": "prepare",
  "line": 1,
  "column": 8
}`}</pre>
            <pre>{`{
  "type": "Identifier",
  "value": "rows",
  "content": "rows",
  "line": 1,
  "column": 13
}`}</pre>
            <pre>{`{
  "type": "Keyword",
  "value": "as",
  "content": "as",
  "line": 1,
  "column": 16
}`}</pre>
            <pre>{`{ 
  "type": "Number", 
  "value": "50", 
  "content": 50, 
  "line": 1, 
  "column": 19 
}`}</pre>
          </div>
        </div>
        <div className={styles.right}>
          <pre>{`{
  "type": "Var",
  "name": "rows",
  "value": { "type": "Literal", "value": 50 }
}`}</pre>
        </div>
      </div>
    </div>
  )
}
