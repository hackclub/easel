import { useEffect, useRef } from 'react'

export default function Game() {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {}, [])

  return <canvas ref={canvasRef}></canvas>
}
