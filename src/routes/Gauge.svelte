<script lang="ts">
  type ColorStop = { offset: number; color: `#${string}` }

  const radius = 100
  const xCenter = 100
  const yCenter = 100

  export let value = 70
  export let min = 0
  export let max = 100
  export let startAngle = -90
  export let endAngle = 90
  export let innerRadius = 60
  export let separatorStep = 10
  export let separatorThickness = 4
  export let gaugeColor: string | ColorStop[] = [
    { offset: 0, color: '#42b983' },
    { offset: 100, color: '#f87272' }
  ]
  export let baseColor: `#${string}` = '#dddddd'
  export let scaleInterval = 5

  $: height =
    Math.abs(endAngle) <= 180 && Math.abs(startAngle) <= 180
      ? Math.max(yCenter, polarToCartesian(radius, startAngle).y, polarToCartesian(radius, endAngle).y)
      : radius * 2

  $: totalAngle = Math.abs(endAngle - startAngle)

  $: separatorPathsValue = separatorPaths(separatorStep, totalAngle, min, max, separatorThickness)

  $: isCircle = Math.abs(totalAngle) === 360

  $: basePath = describePath(radius, startAngle, endAngle)

  $: gaugePath = describePath(radius, getAngle(value), endAngle)

  $: scaleLinesValue = scaleLines(scaleInterval, isCircle, min, max, innerRadius)

  const separatorPaths = (
    separatorStep: number,
    totalAngle: number,
    min: number,
    max: number,
    separatorThickness: number
  ) => {
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

  const polarToCartesian = (radius: number, angle: number) => {
    const angleInRadians = ((angle - 90) * Math.PI) / 180

    return {
      x: xCenter + radius * Math.cos(angleInRadians),
      y: yCenter + radius * Math.sin(angleInRadians)
    }
  }

  const getAngle = (value: number) => {
    const totalValue = max - min || 1

    return (value * totalAngle) / totalValue + startAngle
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

        {#if separatorPathsValue !== null}
          <template>
            {#each separatorPathsValue as separator}
              <path d={separator} fill="black" />
            {/each}
          </template>
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
    {#if scaleLinesValue !== null}
      <template>
        {#each scaleLinesValue as line (line.xE)}
          <line x1={line.xS} y1={line.yS} x2={line.xE} y2={line.yE} stroke-width="1" stroke={baseColor} />
        {/each}
      </template>
    {/if}
    <foreignObject x="0" y="0" width="100%" {height}>
      <slot />
    </foreignObject>
  </svg>
</div>
