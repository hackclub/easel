import React, { PropsWithChildren } from 'react'
import styles from './Node.module.scss'

export default function Node({
  children,
  title
}: {
  children: any
  title: string
}) {
  return (
    <div className="interactive">
      <div className={styles.node}>
        <div>{title}</div>
        <div>{children}</div>
      </div>
    </div>
  )
}
