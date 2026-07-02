
<!-- UNIVERSAL TAG COLLECTION -->
<!-- HEADING 1:             <h1 style="color: #EAB308;"><b>INSERT_TEXT_HERE</b></h1>                                                                    -->
<!-- HEADING 2:             <h2 style="color: #F35B13;">INSERT_TEXT_HERE</h2>                                                                           -->
<!-- HEADING 3:             <h3 style="color: #FFBE6F;">INSERT_TEXT_HERE</h3>                                                                           -->
<!-- HEADING 4:             <h4 style="color: #FFFFFF;">INSERT_TEXT_HERE</h4>                                                                           -->
<!-- EMBED TEXT:            <span title="INSERT_EMBED_TEXT_HERE" style="color: #F2F5CD;"><code>INSERT_DISPLAY_TEXT_HERE<code></span>                                -->
<!-- STANDARD TEXT:         <span style="color: #FFE0E0;">INSERT_TEXT_HERE</span>                                                                       -->

<!-- DOCUMENT SPECIFIC TAG COLLECTION -->



<h1 style="color: #EAB308;"><b>TICKERS</b></h1>
<p style="color: #FFE0E0;">Text</p>
<h2 style="color: #F35B13;">How To Use The Package?</h2>

<h3 style="color: #FFBE6F;">The Recommended Way</h3>
<ol>
    <li>
        <p style="color: #FFE0E0;">Add the package to your project.</p>
        <pre><code class="language-bash">cargo add mirth_engine_tickers</code></pre>
    </li>
    <li><p style="color: #FFE0E0;">Add <code>.add_plugin(Tickers{})</code> to your Bevy app.</p></li>
    <li><p style="color: #FFE0E0;">You can now make use of the datatypes that this package offers.</p></li>
</ol>

<h3 style="color: #FFBE6F;">The <i>"I Choose Features"</i> Way</h3>
<ol>
    <li>
        <p style="color: #FFE0E0;">Add the package to your project with the default features disabled, then explicitly choose which features you want.</p>
        <ul>
            <li><p style="color: #FFE0E0;"><code>ticker_reflect</code> : Will cause all ticker types to reflect. Will add the <code>bevy_reflect</code> dependency and allow for the Tickers plugin to be usable.</p></li>
            <li><p style="color: #FFE0E0;"><code>ticker_systems</code> : Will turn on all systems for every ticker type. Will add the <code>bevy_time</code> dependency and allow for the Tickers plugin to be usable.</p></li>
            <li><p style="color: #FFE0E0;"><code>ticker_serialize</code> : Will make it so that ticker types can be serialized and deserialized. Will enable <code>ticker_reflect</code>, add the <code>serde</code> dependency, and allow for the Tickers plugin to be usable.</p></li>
        </ul>
        <pre><code class="language-bash">cargo add mirth_engine_tickers --no-default-features --features ticker_reflect, ticker_systems, ticker_serialize</code></pre>
    </li>
    <li><p style="color: #FFE0E0;">If you added any of the features, add <code>.add_plugin(Tickers{})</code> to your Bevy app.</p></li>
    <li><p style="color: #FFE0E0;">You can now make use of the datatypes that this package offers.</p></li>
</ol>

<h3 style="color: #FFBE6F;">The <i>"I Will Do Everything Myself"</i> Way</h3>
<ol>
    <li>
        <p style="color: #FFE0E0;">Add the package to your project with the default features disabled.</p>
        <pre><code class="language-bash">cargo add mirth_engine_tickers --no-default-features</code></pre>
    </li>
    <li><p style="color: #FFE0E0;">You can now make use of the datatypes that this package offers, but you will need to create ticking systems, reflection registrations, and serialization capability yourself.</p></li>
</ol>
<h2 style="color: #F35B13;">What Are The Datatypes This Package Offers?</h2>
<p style="color: #FFE0E0;">Hover over any of the following type names</p>
<ul>
    <li>
        <code style="color: #F2F5CD;">Tickers</code>
        <ul>
            <li><p style="color: #FFE0E0;">A Bevy plugin used to activate ticker systems and reflections for ticker types.</p></li>
            <li><p style="color: #FFE0E0;">Use <code>.add_plugin(Tickers{})</code> inside a Bevy app to enable ticker systems and reflections.</p></li>
        </ul>
    </li>
    <br>
    <li>
        <code title="New Text" style="color: #F2F5CD;"><b>Ticker</b></code>
        <ul>
            <li><p style="color: #FFE0E0;">A type used to track the time between events.</p></li>
            <li>
                <p style="color: #FFE0E0;">Holds the following fields:</p>
                <ul>
                    <li><p style="color: #FFE0E0;"><code>start_value</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>current_value</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>end_value</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>time_interval</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>is_paused</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>is_ticking_up</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>is_handling_time_spikes</code></p></li>
                    <li><p style="color: #FFE0E0;"><code>behavior</code></p></li>
                </ul>
            </li>
            <li>
                <code style="color: #F2F5CD;">TickerBehaviors</code>
                <ul>
                    <li>
                        <p style="color: #FFE0E0;">An enum that establishes the different types of behavior a <code>Ticker</code> can use.</p>
                        <ul>
                            <li><p style="color: #FFE0E0;">Supports Looper, MutLooper, Oneshot, MutOneshot, and Freezing behaviors.</p></li>
                        </ul>
                    </li>
                </ul>
            </li>
            <li>
                <code style="color: #F2F5CD;">TickerValue</code>
                <ul>
                    <li>
                        <p style="color: #FFE0E0;">A trait for implementing a generic to define integer primitives a <code>Ticker</code> can potentially store for its boundary values and current value.</p>
                        <ul>
                            <li><p style="color: #FFE0E0;">Supports i8, i16, and i32.</p></li>
                        </ul>
                    </li>
                </ul>
            </li>
            <li>
                <code style="color: #F2F5CD;">TickerPrecision</code>
                <ul>
                    <li>
                        <p style="color: #FFE0E0;">A trait for implementing a generic to define float types a <code>Ticker</code> can potentially use for its precision in tracking time.</p>
                        <ul>
                            <li><p style="color: #FFE0E0;">Supports f16, f32, and f64.</p></li>
                        </ul>
                    </li>
                </ul>
            </li>
            <li>
                <code style="color: #F2F5CD;">TickerFloatBridge</code>
                <ul>
                    <li>
                        <p style="color: #FFE0E0;">A trait which grants the ability for f16, f32, and f64 to be passed in for the float fields of a <code>Ticker</code> constructor, no matter the precision a ticker is set to.</p>
                        <ul>
                            <li><p style="color: #FFE0E0;">Eases usage of <code>Ticker</code> constructor methods.</p></li>
                        </ul>
                    </li>
                </ul>
            </li>
        </ul>
    </li>
</ul>
<h2 style="color: #F35B13;">What Can I Do With The <code>Ticker</code> Datatype</h2>
<p style="color: #FFE0E0;">Hover over any of the following method names to reveal more information about them.</p>
<ul>
    <li><span title="INSERT_EMBED_TEXT_HERE" style="color: #F2F5CD;">INSERT_DISPLAY_TEXT_HERE</span></li>
</ul>


<!-- ################################################################################################################################################## -->
<!-- WHAT ARE THE DATATYPES THIS PACKAGE OFFERS? -->
<h2 style="color: #F35B13;">What Are The Datatypes This Package Offers?</h2>
<span style="color: #FFE0E0;">Hover over any of the following type names
- `Tickers`
  - <span style="color: #FFE0E0;">A Bevy plugin used to activate ticker systems and reflections for ticker types.
  - <span style="color: #FFE0E0;">Use `.add_plugin(Tickers{})` inside a Bevy app to enable ticker systems and reflections.


- `Ticker`
  - <span style="color: #FFE0E0;">A type used to track the time between events.
  - <span style="color: #FFE0E0;">Holds the following fields:
    - start_value
    - current_value
    - end_value
    - time_interval
    - is_paused
    - is_ticking_up
    - is_handling_time_spikes
    - behavior
  - <span style="color: #FFE0E0;">Is supported by the following types:
    - `TickerBehaviors`
      - <span style="color: #FFE0E0;">An enum that establishes the different types of behavior a `Ticker` can use.
        - <span style="color: #FFE0E0;">Supports Looper, MutLooper, Oneshot, MutOneshot, and Freezing behaviors.
    - `TickerValue`
      - <span style="color: #FFE0E0;">A trait for implementing a generic to define integer primitives a `Ticker` can potentially store for its boundary values and current value.
        - <span style="color: #FFE0E0;">Supports i8, i16, and i32.
    - `TickerPrecision`
      - <span style="color: #FFE0E0;">A trait for implementing a generic to define float types a `Ticker` can potentially use for its precision in tracking time.
        - <span style="color: #FFE0E0;">Supports f16, f32, and f64.
    - `TickerFloatBridge`
      - <span style="color: #FFE0E0;">A trait which grants the ability for f16, f32, and f64 to be passed in for the float fields of a `Ticker` constructor, no matter the precision a ticker is set to.
        - <span style="color: #FFE0E0;">Eases usage of `Ticker` constructor methods.
<!-- ################################################################################################################################################## -->



<!-- ################################################################################################################################################## -->
<!-- WHAT CAN I DO WITH THE TICKER DATATYPE? -->
<h2 style="color: #F35B13;">What Can I Do With The `Ticker` Datatype</h2>
<span style="color: #FFE0E0;">Hover over any of the following method names to reveal more information about them.
- <span title="INSERT_EMBED_TEXT_HERE" style="color: #FFE0E0;">INSERT_DISPLAY_TEXT_HERE</span>
<!-- ################################################################################################################################################## -->
