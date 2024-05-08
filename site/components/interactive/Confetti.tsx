import ReactCanvasConfetti from 'react-canvas-confetti'
import { useCallback, useEffect, useRef, useState } from 'react'

function randomInRange(min: number, max: number) {
  return Math.random() * (max - min) + min
}

const canvasStyles = {
  position: 'fixed',
  pointerEvents: 'none',
  width: '100vw',
  height: '100vh',
  top: 0,
  left: 0,
  zIndex: 9999
}

export default function Confetti() {
  const refAnimationInstance = useRef(null)

  function getFireworkAnimationSettings(originXA, originXB) {
    return {
      startVelocity: 35,
      spread: 360,
      ticks: 100,
      zIndex: 0,
      particleCount: 100,
      origin: {
        x: randomInRange(originXA, originXB),
        y: Math.random()
      }
    }
  }

  const getInstance = useCallback(instance => {
    refAnimationInstance.current = instance
  })

  const nextFireworkTickAnimation = useCallback(() => {
    if (refAnimationInstance.current) {
      refAnimationInstance.current(getFireworkAnimationSettings(0.2, 0.3))
      refAnimationInstance.current(getFireworkAnimationSettings(0.7, 0.9))
    }
  }, [])

  useEffect(() => {
    setTimeout(nextFireworkTickAnimation, 1000)
    setTimeout(nextFireworkTickAnimation, 1800)
    setTimeout(nextFireworkTickAnimation, 2600)
    setTimeout(nextFireworkTickAnimation, 3400)
    setTimeout(nextFireworkTickAnimation, 4200)
  }, [])

  return (
    <>
      <ReactCanvasConfetti refConfetti={getInstance} style={canvasStyles} />
    </>
  )
}
