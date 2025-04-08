<?xml version="1.0" encoding="ISO-8859-1"?>
<!--
Custom Rose Pine themed NMAP XSL stylesheet
Based on the original nmap.xsl
-->
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:output method="html" indent="yes" encoding="UTF-8" doctype-system="about:legacy-compat"/>
  <xsl:template match="/">
    <html lang="en">
      <head>
        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <meta http-equiv="content-type" content="text/html; charset=utf-8"/>
        <title>Nmap Scan Report - Rose Pine Theme</title>
        <style>
          body {
            font-family: 'JetBrains Mono', monospace, sans-serif;
            background-color: #191724; /* Rose Pine base */
            color: #e0def4; /* Rose Pine text */
            margin: 0;
            padding: 20px;
            line-height: 1.5;
          }
          h1, h2, h3, h4, h5, h6 {
            color: #ebbcba; /* Rose Pine rose */
            margin-top: 12px;
            margin-bottom: 8px;
          }
          h1 {
            font-size: 24px;
            border-bottom: 1px solid #6e6a86; /* Rose Pine muted */
            padding-bottom: 8px;
          }
          h2 {
            font-size: 20px;
          }
          h3 {
            font-size: 18px;
          }
          table {
            width: 100%;
            border-collapse: collapse;
            margin-bottom: 16px;
            border-radius: 6px;
            overflow: hidden;
          }
          th {
            background-color: #26233a; /* Rose Pine overlay */
            color: #9ccfd8; /* Rose Pine foam */
            text-align: left;
            padding: 8px 12px;
          }
          td {
            padding: 8px 12px;
            border: 1px solid #6e6a86; /* Rose Pine muted */
          }
          tr:nth-child(odd) {
            background-color: #1f1d2e; /* Rose Pine surface */
          }
          tr:hover {
            background-color: #26233a; /* Rose Pine overlay */
          }
          tr.head {
            background-color: #26233a; /* Rose Pine overlay */
          }
          .up {
            color: #31748f; /* Rose Pine pine */
            font-weight: bold;
          }
          .down {
            color: #eb6f92; /* Rose Pine love */
            font-weight: bold;
          }
          .print-only {
            display: none;
          }
          @media print {
            .print-only {
              display: block;
            }
            body {
              background-color: white;
              color: black;
            }
          }
          .highlight {
            background-color: #2a273f; /* Slightly lighter than overlay */
            border-left: 3px solid #c4a7e7; /* Rose Pine iris */
            padding: 8px 12px;
            margin: 8px 0;
          }
          a {
            color: #c4a7e7; /* Rose Pine iris */
            text-decoration: none;
          }
          a:hover {
            text-decoration: underline;
          }
          .summary-table {
            margin-top: 16px;
            border: 1px solid #6e6a86;
          }
          pre {
            background-color: #1f1d2e; /* Rose Pine surface */
            padding: 10px;
            border-radius: 6px;
            overflow-x: auto;
          }
          .service-version {
            color: #f6c177; /* Rose Pine gold */
          }
          .port-number {
            color: #9ccfd8; /* Rose Pine foam */
            font-weight: bold;
          }
          .meta-info {
            font-style: italic;
            color: #908caa; /* Rose Pine subtle */
          }
        </style>
      </head>
      <body>
        <h1>Nmap Scan Report - <xsl:value-of select="/nmaprun/@startstr" /></h1>
        <div class="highlight">
          <h2>Scan Summary</h2>
          <p>
            <strong>Command:</strong> <xsl:value-of select="/nmaprun/@args" /><br/>
            <strong>Start Time:</strong> <xsl:value-of select="/nmaprun/@startstr" /><br/>
            <strong>Scan Type:</strong> <xsl:value-of select="/nmaprun/scaninfo/@type" /><br/>
            <strong>Protocol:</strong> <xsl:value-of select="/nmaprun/scaninfo/@protocol" />
          </p>
        </div>

        <!-- For each host -->
        <xsl:for-each select="/nmaprun/host">
          <div class="host-section">
            <h2>
              Host: 
              <xsl:choose>
                <xsl:when test="status/@state = 'up'">
                  <span class="up">UP</span>
                </xsl:when>
                <xsl:otherwise>
                  <span class="down">DOWN</span>
                </xsl:otherwise>
              </xsl:choose>
              
              <xsl:choose>
                <xsl:when test="count(hostnames/hostname) > 0">
                  <xsl:value-of select="hostnames/hostname/@name"/> (<xsl:value-of select="address/@addr"/>)
                </xsl:when>
                <xsl:otherwise>
                  <xsl:value-of select="address/@addr"/>
                </xsl:otherwise>
              </xsl:choose>
            </h2>
            
            <!-- Host addresses -->
            <h3>Addresses</h3>
            <table>
              <tr class="head">
                <th>Type</th>
                <th>Address</th>
              </tr>
              <xsl:for-each select="address">
                <tr>
                  <td><xsl:value-of select="@addrtype"/></td>
                  <td><xsl:value-of select="@addr"/></td>
                </tr>
              </xsl:for-each>
            </table>
            
            <!-- Host names -->
            <xsl:if test="count(hostnames/hostname) > 0">
              <h3>Hostnames</h3>
              <table>
                <tr class="head">
                  <th>Type</th>
                  <th>Hostname</th>
                </tr>
                <xsl:for-each select="hostnames/hostname">
                  <tr>
                    <td><xsl:value-of select="@type"/></td>
                    <td><xsl:value-of select="@name"/></td>
                  </tr>
                </xsl:for-each>
              </table>
            </xsl:if>
            
            <!-- Port table -->
            <xsl:if test="count(ports/port) > 0">
              <h3>Ports</h3>
              <table>
                <tr class="head">
                  <th>Port</th>
                  <th>State</th>
                  <th>Service</th>
                  <th>Reason</th>
                  <th>Version</th>
                </tr>
                
                <xsl:for-each select="ports/port">
                  <tr>
                    <td class="port-number">
                      <xsl:value-of select="@portid"/>/<xsl:value-of select="@protocol"/>
                    </td>
                    <td>
                      <xsl:choose>
                        <xsl:when test="state/@state = 'open'">
                          <span class="up"><xsl:value-of select="state/@state"/></span>
                        </xsl:when>
                        <xsl:when test="state/@state = 'filtered'">
                          <span class="down"><xsl:value-of select="state/@state"/></span>
                        </xsl:when>
                        <xsl:otherwise>
                          <xsl:value-of select="state/@state"/>
                        </xsl:otherwise>
                      </xsl:choose>
                    </td>
                    <td>
                      <xsl:value-of select="service/@name"/>
                    </td>
                    <td>
                      <xsl:value-of select="state/@reason"/>
                    </td>
                    <td class="service-version">
                      <xsl:if test="service/@product">
                        <xsl:value-of select="service/@product"/>
                        <xsl:if test="service/@version">
                          <xsl:text> </xsl:text>
                          <xsl:value-of select="service/@version"/>
                        </xsl:if>
                        <xsl:if test="service/@extrainfo">
                          <xsl:text> (</xsl:text>
                          <xsl:value-of select="service/@extrainfo"/>
                          <xsl:text>)</xsl:text>
                        </xsl:if>
                      </xsl:if>
                    </td>
                  </tr>
                  
                  <!-- Handle scripts -->
                  <xsl:if test="count(script) > 0">
                    <tr>
                      <td colspan="5">
                        <xsl:for-each select="script">
                          <pre>
                            <strong><xsl:value-of select="@id"/>:</strong>
                            <xsl:value-of select="@output"/>
                          </pre>
                        </xsl:for-each>
                      </td>
                    </tr>
                  </xsl:if>
                </xsl:for-each>
              </table>
            </xsl:if>
            
            <!-- OS Detection -->
            <xsl:if test="count(os/osmatch) > 0">
              <h3>OS Detection</h3>
              <table>
                <tr class="head">
                  <th>OS</th>
                  <th>Accuracy</th>
                  <th>Type</th>
                </tr>
                <xsl:for-each select="os/osmatch">
                  <tr>
                    <td><xsl:value-of select="@name"/></td>
                    <td><xsl:value-of select="@accuracy"/>%</td>
                    <td>
                      <xsl:for-each select="osclass">
                        <xsl:value-of select="@type"/>
                        <xsl:if test="position() != last()">
                          <xsl:text>, </xsl:text>
                        </xsl:if>
                      </xsl:for-each>
                    </td>
                  </tr>
                </xsl:for-each>
              </table>
            </xsl:if>
            
          </div> <!-- End host section -->
        </xsl:for-each>
        
        <div class="meta-info">
          <p>
            Generated with <xsl:value-of select="/nmaprun/scaninfo/@version"/> on <xsl:value-of select="/nmaprun/@startstr"/><br/>
            Scan completed in <xsl:value-of select="/nmaprun/@runstats/finished/@elapsed"/> seconds
          </p>
        </div>
      </body>
    </html>
  </xsl:template>
</xsl:stylesheet>