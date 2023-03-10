import React, { FunctionComponent } from 'react'
import { TextButton } from '../../atoms'
import styles from './style.module.css'

interface NameUnavailableProps {
  name: string
  handleReset: () => void
}

const NameUnavailable: FunctionComponent<NameUnavailableProps> = props => {
  return (
    <div>
      <p className={styles.emphasized}>
        <b>{props.name}.xlm</b>
      </p>

      <p className={styles.p}>is already registered :(</p>

      <TextButton title={'Try another name'} onClick={props.handleReset} />
    </div>
  )
}

export { NameUnavailable }
