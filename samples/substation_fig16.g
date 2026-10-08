<?xml version="1.0" encoding="UTF-8"?>
<G type="substation" viewbox="0, 0, 1200, 800" background="245, 245, 245" app="SCADA" context="realtime">
  <Substation id="Substation_South" loc="50, 50 1100, 700" data="#Substation001" show="0,0,0,0" grid="G1">
    <VoltageLevel id="500kV" loc="50, 50 1000, 200" data="#VL_500kV" show="0,5,0,0" volt="500kV">
      <BusbarSection id="Bus_500_1" loc="100, 100 800, 10" data="#Bus5001" show="0,5,0,0"/>
      <Bay id="Bay_501" loc="150, 120 100, 120" data="#Bay501" show="0,5,0,0">
        <Breaker id="CB_501" loc="170, 140 20, 20" data="#CB501" show="0,5,0,0" A="F1"/>
        <Disconnector id="DIS_501A" loc="170, 120 20, 20" data="#DIS501A" show="0,5,0,0"/>
        <DText id="DText_CB501_P" loc="200, 145" data="#CB501_MW" show="0,5,0,0" value="450.2 MW"/>
      </Bay>
    </VoltageLevel>
    <VoltageLevel id="220kV" loc="50, 300 1000, 200" data="#VL_220kV" show="0,8,0,0" volt="220kV">
      <BusbarSection id="Bus_220_1" loc="100, 350 800, 10" data="#Bus2201" show="0,8,0,0"/>
      <Bay id="Bay_201" loc="150, 370 100, 120" data="#Bay201" show="0,8,0,0">
        <Breaker id="CB_201" loc="170, 390 20, 20" data="#CB201" show="0,8,0,0"/>
        <DText id="DText_CB201_P" loc="200, 395" data="#CB201_MW" show="0,8,0,0" value="180.5 MW"/>
      </Bay>
    </VoltageLevel>
    <PowerTransformer id="Trans_Main_1" loc="500, 230 40, 50" data="#T01" show="0,5,0,0"/>
    <Link points="180,160 180,230" connect="glue0,CB_501;glue0,Trans_Main_1" show="0,5,0,0"/>
  </Substation>
  <DataList type="state" num="10" start="1" end="10"/>
  <DataList type="measure" num="20" start="11" end="30"/>
</G>
