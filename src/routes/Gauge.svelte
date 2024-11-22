<script lang="ts">
  import { tweened } from 'svelte/motion'
  import { cubicOut } from 'svelte/easing'

  /*
  This gauge is based on the amazing work of [vue-svg-gauge](https://github.com/hellocomet/vue-svg-gauge/).
  */

  type ColorStop = { offset: number; color: `#${string}` }

  const radius = 100
  const xCenter = 100
  const yCenter = 100

  interface Props {
    value?: number
    min?: number
    max?: number
    startAngle?: number
    endAngle?: number
    innerRadius?: number
    separatorStep?: number
    separatorThickness?: number
    gaugeColor?: string | ColorStop[]
    baseColor?: `#${string}`
    scaleInterval?: number
    children?: import('svelte').Snippet
  }

  const {
    value = 70,
    min = 0,
    max = 100,
    startAngle = -90,
    endAngle = 90,
    innerRadius = 60,
    separatorStep = 10,
    separatorThickness = 4,
    gaugeColor = [
      { offset: 0, color: '#42b983' },
      { offset: 100, color: '#f87272' }
    ],
    baseColor = '#dddddd',
    scaleInterval = 5,
    children
  }: Props = $props()

  const tweenedAngle = tweened(value, {
    duration: 200,
    easing: cubicOut
  })

  $effect(() => {
    tweenedAngle.set(value)
  })

  const polarToCartesian = (radius: number, angle: number) => {
    const angleInRadians = ((angle - 90) * Math.PI) / 180

    return {
      x: xCenter + radius * Math.cos(angleInRadians),
      y: yCenter + radius * Math.sin(angleInRadians)
    }
  }

  const getAngle = (value: number) => {
    const clampedAngle = Math.min(Math.max(value, min), max)
    const totalValue = max - min || 1

    return (clampedAngle * totalAngle) / totalValue + startAngle
  }

  const describePath = (radius: number, startAngle: number, endAngle: number) => {
    const start = polarToCartesian(radius, endAngle)
    const end = polarToCartesian(radius, startAngle)

    const largeArcFlag = endAngle - startAngle <= 180 ? '0' : '1'

    const d = [
      'M',
      start.x,
      start.y,
      'A',
      radius,
      radius,
      0,
      largeArcFlag,
      0,
      end.x,
      end.y,
      'L',
      xCenter,
      yCenter
    ].join(' ')

    return d
  }

  const separatorPaths = (separatorStep: number, min: number, max: number, separatorThickness: number) => {
    if (separatorStep > 0) {
      const paths = []
      // If the gauge is a circle, this will add a separator at the start
      let i = isCircle ? min : min + separatorStep

      for (i; i < max; i += separatorStep) {
        const angle = getAngle(i)
        const halfAngle = separatorThickness / 2

        paths.push(describePath(radius + 2, angle - halfAngle, angle + halfAngle))
      }

      return paths
    }

    return null
  }

  const scaleLines = (scaleInterval: number, isCircle: boolean, min: number, max: number, innerRadius: number) => {
    if (scaleInterval > 0) {
      const lines = []
      // if gauge is a circle, remove the first scale
      let i = isCircle ? min + scaleInterval : min

      for (i; i < max + scaleInterval; i += scaleInterval) {
        const angle = getAngle(i)
        const startCoordinate = polarToCartesian(innerRadius - 4, angle)
        const endCoordinate = polarToCartesian(innerRadius - 8, angle)

        lines.push({
          xS: startCoordinate.x,
          yS: startCoordinate.y,
          xE: endCoordinate.x,
          yE: endCoordinate.y
        })
      }

      return lines
    }

    return null
  }

  const height = $derived(
    Math.abs(endAngle) <= 180 && Math.abs(startAngle) <= 180
      ? Math.max(yCenter, polarToCartesian(radius, startAngle).y, polarToCartesian(radius, endAngle).y)
      : radius * 2
  )
  const totalAngle = $derived(Math.abs(endAngle - startAngle))
  const separatorPathsValue = $derived(separatorPaths(separatorStep, min, max, separatorThickness))
  const isCircle = $derived(Math.abs(totalAngle) === 360)
  const basePath = $derived(describePath(radius, startAngle, endAngle))
  const gaugePath = $derived(describePath(radius, getAngle($tweenedAngle), endAngle))
  const scaleLinesValue = $derived(scaleLines(scaleInterval, isCircle, min, max, innerRadius))
</script>

<div>
  <svg viewBox={`0 0 ${radius * 2} ${height}`} height="100%" width="100%" xmlns="http://www.w3.org/2000/svg">
    <defs>
      <filter id="innershadow">
        <feFlood flood-color="#c7c6c6" />
        <feComposite in2="SourceAlpha" operator="out" />
        <feGaussianBlur stdDeviation="2" result="blur" />
        <feComposite operator="atop" in2="SourceGraphic" />
      </filter>
      {#if Array.isArray(gaugeColor)}
        <linearGradient id="gaugeGradient">
          {#each gaugeColor as color (color.offset)}
            <stop offset={`${color.offset}%`} stop-color={color.color} />
          {/each}
        </linearGradient>
      {/if}
      <mask id="innerCircle">
        <circle r={radius - 0.5} cx={xCenter} cy={yCenter} fill="white" />

        <circle r={innerRadius} cx={xCenter} cy={yCenter} fill="black" />

        {#if Array.isArray(separatorPathsValue)}
          {#each separatorPathsValue as separator}
            <path d={separator} fill="black" />
          {/each}
        {/if}
      </mask>
    </defs>
    <g mask="url(#innerCircle)">
      {#if isCircle}
        <circle
          r={radius}
          cx={xCenter}
          cy={yCenter}
          fill={Array.isArray(gaugeColor) ? 'url(#gaugeGradient)' : gaugeColor}
        />
      {:else}
        <path d={basePath} fill={Array.isArray(gaugeColor) ? 'url(#gaugeGradient)' : gaugeColor} />
      {/if}

      {#if isCircle && value === min}
        <circle r={radius} cx={xCenter} cy={yCenter} fill={baseColor} />
      {:else}
        <path d={gaugePath} fill={baseColor} filter="url(#innershadow)" />
      {/if}
    </g>
    {#if Array.isArray(scaleLinesValue)}
      {#each scaleLinesValue as line (line.xE)}
        <line x1={line.xS} y1={line.yS} x2={line.xE} y2={line.yE} stroke-width="1" stroke={baseColor} />
      {/each}
    {/if}
    <foreignObject x="0" y="0" width="100%" {height}>
      {@render children?.()}
    </foreignObject>
  </svg>
</div>
