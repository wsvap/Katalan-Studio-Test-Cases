<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_var require          base</name>
   <tag></tag>
   <elementGuidId>21d4e715-9927-4256-ba32-0bf5aa680114</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
    var require = {
        &quot;baseUrl&quot;: &quot;http://uk.nordtek.dev.erise.hu/pub/static/version1531145109/frontend/Infortis/ultimo/en_US&quot;
    };

        





Nordtek























        
    
    #katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}
    
        

    try {
        if (!window.localStorage || !window.sessionStorage) {
            throw new Error();
        }

        localStorage.setItem('storage_test', 1);
        localStorage.removeItem('storage_test');
    } catch(e) {
        (function () {
            var Storage = function (type) {
                var data;

                function createCookie(name, value, days) {
                    var date, expires;

                    if (days) {
                        date = new Date();
                        date.setTime(date.getTime()+(days * 24 * 60 * 60 * 1000));
                        expires = '; expires=' + date.toGMTString();
                    } else {
                        expires = '';
                    }
                    document.cookie = name + '=' + value+expires+'; path=/';
                }

                function readCookie(name) {
                    var nameEQ = name + '=',
                        ca = document.cookie.split(';'),
                        i = 0,
                        c;

                    for (i=0; i &lt; ca.length; i++) {
                        c = ca[i];

                        while (c.charAt(0) === ' ') {
                            c = c.substring(1,c.length);
                        }

                        if (c.indexOf(nameEQ) === 0) {
                            return c.substring(nameEQ.length, c.length);
                        }
                    }

                    return null;
                }

                function setData(data) {
                    data = encodeURIComponent(JSON.stringify(data));
                    createCookie(type === 'session' ? getSessionName() : 'localStorage', data, 365);
                }

                function clearData() {
                    createCookie(type === 'session' ? getSessionName() : 'localStorage', '', 365);
                }

                function getData() {
                    var data = type === 'session' ? readCookie(getSessionName()) : readCookie('localStorage');

                    return data ? JSON.parse(decodeURIComponent(data)) : {};
                }

                function getSessionName() {
                    if (!window.name) {
                        window.name = new Date().getTime();
                    }

                    return 'sessionStorage' + window.name;
                }

                data = getData();

                return {
                    length: 0,
                    clear: function () {
                        data = {};
                        this.length = 0;
                        clearData();
                    },

                    getItem: function (key) {
                        return data[key] === undefined ? null : data[key];
                    },

                    key: function (i) {
                        var ctr = 0,
                            k;

                        for (k in data) {
                            if (ctr.toString() === i.toString()) {
                                return k;
                            } else {
                                ctr++
                            }
                        }

                        return null;
                    },

                    removeItem: function (key) {
                        delete data[key];
                        this.length--;
                        setData(data);
                    },

                    setItem: function (key, value) {
                        data[key] = value.toString();
                        this.length++;
                        setData(data);
                    }
                };
            };

            window.localStorage.__proto__ = window.localStorage = new Storage('local');
            window.sessionStorage.__proto__ = window.sessionStorage = new Storage('session');
        })();
    }



    window.ajaxCartLoaderOptions = {&quot;icon&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/pub\/static\/version1531145109\/frontend\/Infortis\/ultimo\/en_US\/images\/loader-1.gif&quot;,&quot;texts&quot;:{&quot;loaderText&quot;:&quot;Loading...&quot;,&quot;imgAlt&quot;:&quot;Loading...&quot;}};
    
        require.config({
            deps: [
                'jquery',
                'mage/translate',
                'jquery/jquery-storageapi'
            ],
            callback: function ($) {
                'use strict';

                var dependencies = [],
                    versionObj;

                $.initNamespaceStorage('mage-translation-storage');
                $.initNamespaceStorage('mage-translation-file-version');
                versionObj = $.localStorage.get('mage-translation-file-version');

                
                if (versionObj.version !== '182481f4cb15f9cebe1411945e23aacd5ce1b377') {
                    dependencies.push(
                        'text!js-translation.json'
                    );

                }

                require.config({
                    deps: dependencies,
                    callback: function (string) {
                        if (typeof string === 'string') {
                            $.mage.translate.add(JSON.parse(string));
                            $.localStorage.set('mage-translation-storage', string);
                            $.localStorage.set(
                                'mage-translation-file-version',
                                {
                                    version: '182481f4cb15f9cebe1411945e23aacd5ce1b377'
                                }
                            );
                        } else {
                            $.mage.translate.add($.localStorage.get('mage-translation-storage'));
                        }
                    }
                });
            }
        });
    


    
        &lt;div class=&quot;message global noscript&quot;>
            &lt;div class=&quot;content&quot;>
                &lt;p>
                    &lt;strong>JavaScript seems to be disabled in your browser.&lt;/strong>
                    &lt;span>For the best experience on our site, be sure to turn on Javascript in your browser.&lt;/span>
                &lt;/p>
            &lt;/div>
        &lt;/div>
    
    
        
            
                We use cookies to make your experience better.
                To comply with the new e-Privacy directive, we need to ask for your consent to set the cookies.
                Learn more.            
            
                
                    Allow Cookies
                
            
        
    
    







        
    

        
            
                

                    
                                                                        
    
        
            EUR
            
        
    
    
                                    
                    GBP - British Pound Sterling
                
                                            


                                                                    
                 
             
         

        
            
                

                    
                    
                    
                            
        
            Nordtek Ltd.
            
        
    
                                            
                    

                    
                    

                                                    
                                
                                Menu
                            
                        
                                                    
                                
                                Search
                            
                        
                                                    
                                
                                Account
                            
                        
                        
                        
                                                    
                            
                        
                                                                            
                            
                        
                                                    
                                
                            

                                                    
                            
                        
                                                    
                                
                        
                        
                            

                     

                 
             
         

     
    
    
    
        
            

                Skip to Content

                                
                

                    
                    
                                            


Home
Our references
Shipping
PackChannel
FAQ
Contact


 Call +44 14 03 887 124



                    
                                            
                            
                        
                    
                                            
                            
    
        Compare Products        
    


                        
                    
                 

                

                                                                            
    Select Website

    
                        
                
                    UK Website
                
            
                                                                                                                                                                            
                                                                        
                    
                        HU Website                    
                
                                                                
                    
                        FR Website                    
                
                                                                
                    
                        DE Website                    
                
                                                                
                    
                        IT Website                    
                
                                                                
                    
                        ES Website                    
                
                                                                
                    
                        PL Website                    
                
                                                                
                    
                        CZ Website                    
                
                                                                
                    
                        SK Website                    
                
                                                                
                    
                        RO Website                    
                
                                                                
                    
                        FI Website                    
                
                                                                
                    
                        HR Website                    
                
                                                                
                    
                        BG Website                    
                
                                                                
                    
                        NL Website                    
                
                        
    
    


                                                    
    
        
            EUR
            
        
    
    
                                    
                    GBP - British Pound Sterling
                
                                            


                                                    
    

        
        
        
        
                
        
        
        
                
        
                    
        Account
    
        Sign In    

Sign Up            
        
        
     


                                            
                    
                    
                 

             
         
     

    
        
            

                
                                

                                            
                        
                                                                                                                                
        
            Nordtek Ltd.
            
        
    

                                                                                     
                    
                                            
                        
                                                                                                                            
    
        Search
        
            
                
                                        
                        
                        
                                            
                
                
                    
                        Search
                    
                
            
        
     

                                                                                     
                    
                                            
                        
                                                                                                                            
    
        
            
            Inquiry list
            
                0
            
        
    

                                                                    
        
        
            
            Basket
            
                0
                
                
                
            
            
        
    
            
            
                

    
        My Cart
        0
    



    
        Close
    

    

    

    
        You have no items in your shopping cart.
        
    

    

    
        


    



            
                    
        
        window.checkout = {&quot;shoppingCartUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/checkout\/cart\/&quot;,&quot;checkoutUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/checkout\/&quot;,&quot;updateItemQtyUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/checkout\/sidebar\/updateItemQty\/&quot;,&quot;removeItemUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/checkout\/sidebar\/removeItem\/&quot;,&quot;imageTemplate&quot;:&quot;Magento_Catalog\/product\/image_with_borders&quot;,&quot;baseUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/&quot;,&quot;minicartMaxItemsVisible&quot;:5,&quot;websiteId&quot;:&quot;1&quot;,&quot;maxItemsToDisplay&quot;:10,&quot;customerLoginUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/customer\/account\/login\/&quot;,&quot;isRedirectRequired&quot;:false,&quot;autocomplete&quot;:&quot;off&quot;,&quot;captcha&quot;:{&quot;user_login&quot;:{&quot;isCaseSensitive&quot;:false,&quot;imageHeight&quot;:50,&quot;imageSrc&quot;:&quot;&quot;,&quot;refreshUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/captcha\/refresh\/&quot;,&quot;isRequired&quot;:false},&quot;guest_checkout&quot;:{&quot;isCaseSensitive&quot;:false,&quot;imageHeight&quot;:50,&quot;imageSrc&quot;:&quot;&quot;,&quot;refreshUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/captcha\/refresh\/&quot;,&quot;isRequired&quot;:false}}};
    
    
    
        //&lt;![CDATA[
        requirejs(['jquery'], function(jQuery) {
            jQuery(function($) {
                var miniCartBlock = $('#minicart');
                miniCartBlock.on('dropdown-block-opened', function(e) {
                    if (miniCartBlock.data('mage-sidebar'))
                    {
                        miniCartBlock.sidebar('update');
                    }
                });
            });
        }); //end: requirejs
        //]]>
    

                                                                                     
                                        
                 

             
         
     

            
    
        
            ALL PRODUCTDispensersContainersCapsTransport packagingFood packagingOthersSprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayerAirless 0-49 ml 50-99 ml 100-249 ml 0-49 ml 50-99 ml 100-249 mlTrigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayerGallon pumpPump High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumps High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumpsFoam pump Discounted foam pump White foam pumps Cosmetic foam pumps Discounted foam pump White foam pumps Cosmetic foam pumpsSprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayerTrigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayerPump High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumps High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumpsFoam pump Discounted foam pump White foam pumps Cosmetic foam pumps Discounted foam pump White foam pumps Cosmetic foam pumpsAirless 0-49 ml 50-99 ml 100-249 ml 0-49 ml 50-99 ml 100-249 mlGallon pumpPlastic bottle Foamer bottle Cosmetic bottle PET bottle Foamer bottle Cosmetic bottle PET bottleGlass products DIN18 DIN22 DIN28 DIN18 DIN22 DIN28Plastic canister HDPE canister PE canister HDPE canister PE canisterSets Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamer Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamerTottle PE tottle HDPE tottle PP tottle PE tottle HDPE tottle PP tottleJar Glass jars 0-49 ml jars Acrilyc jars Glass jars 0-49 ml jars Acrilyc jarsTube PE tubes 0-49 ml tubes 50-99 ml tubes PE tubes 0-49 ml tubes 50-99 ml tubesPill jar Pill jar with tamper evident White pill jars 100-249 ml Pill jar with tamper evident White pill jars 100-249 mlLipstick casePlastic bottle Foamer bottle Cosmetic bottle PET bottle Foamer bottle Cosmetic bottle PET bottleSets Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamer Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamerJar Glass jars 0-49 ml jars Acrilyc jars Glass jars 0-49 ml jars Acrilyc jarsPill jar Pill jar with tamper evident White pill jars 100-249 ml Pill jar with tamper evident White pill jars 100-249 mlGlass products DIN18 DIN22 DIN28 DIN18 DIN22 DIN28Tottle PE tottle HDPE tottle PP tottle PE tottle HDPE tottle PP tottleTube PE tubes 0-49 ml tubes 50-99 ml tubes PE tubes 0-49 ml tubes 50-99 ml tubesLipstick casePlastic canister HDPE canister PE canister HDPE canister PE canisterCardboard boxesPalletCardboard boxesPalletSealing filmFood containers and traysSealing filmFood containers and traysRoll-onAll other accessoriesBucketCosmetic accessoriesBag-in-box, TapRaw materialsMachinesPouchesFragranceDroppersLabelsRoll-onBag-in-box, TapFragranceAll other accessoriesRaw materialsDroppersBucketMachinesLabelsCosmetic accessoriesPouchesDispensersSprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayerAirless 0-49 ml 50-99 ml 100-249 ml 0-49 ml 50-99 ml 100-249 mlTrigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayerGallon pumpPump High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumps High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumpsFoam pump Discounted foam pump White foam pumps Cosmetic foam pumps Discounted foam pump White foam pumps Cosmetic foam pumpsSprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayer Cosmetic sprayer 20/410 sprayer 24/410 sprayerTrigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayer 24/410 Trigger sprayer 28/410 Trigger sprayer 28/415 Trigger sprayerPump High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumps High viscosity dispensing pumps 24/410 pumps 28/410 pumps Dispensing pumpsFoam pump Discounted foam pump White foam pumps Cosmetic foam pumps Discounted foam pump White foam pumps Cosmetic foam pumpsAirless 0-49 ml 50-99 ml 100-249 ml 0-49 ml 50-99 ml 100-249 mlGallon pumpContainersPlastic bottle Foamer bottle Cosmetic bottle PET bottle Foamer bottle Cosmetic bottle PET bottleGlass products DIN18 DIN22 DIN28 DIN18 DIN22 DIN28Plastic canister HDPE canister PE canister HDPE canister PE canisterSets Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamer Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamerTottle PE tottle HDPE tottle PP tottle PE tottle HDPE tottle PP tottleJar Glass jars 0-49 ml jars Acrilyc jars Glass jars 0-49 ml jars Acrilyc jarsTube PE tubes 0-49 ml tubes 50-99 ml tubes PE tubes 0-49 ml tubes 50-99 ml tubesPill jar Pill jar with tamper evident White pill jars 100-249 ml Pill jar with tamper evident White pill jars 100-249 mlLipstick casePlastic bottle Foamer bottle Cosmetic bottle PET bottle Foamer bottle Cosmetic bottle PET bottleSets Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamer Bottle &amp; sprayer Bottle &amp; trigger sprayer Bottle &amp; cap Bottle &amp; pump Bottle &amp; foamerJar Glass jars 0-49 ml jars Acrilyc jars Glass jars 0-49 ml jars Acrilyc jarsPill jar Pill jar with tamper evident White pill jars 100-249 ml Pill jar with tamper evident White pill jars 100-249 mlGlass products DIN18 DIN22 DIN28 DIN18 DIN22 DIN28Tottle PE tottle HDPE tottle PP tottle PE tottle HDPE tottle PP tottleTube PE tubes 0-49 ml tubes 50-99 ml tubes PE tubes 0-49 ml tubes 50-99 ml tubesLipstick casePlastic canister HDPE canister PE canister HDPE canister PE canisterCapsTransport packagingCardboard boxesPalletCardboard boxesPalletFood packagingSealing filmFood containers and traysSealing filmFood containers and traysOthersRoll-onAll other accessoriesBucketCosmetic accessoriesBag-in-box, TapRaw materialsMachinesPouchesFragranceDroppersLabelsRoll-onBag-in-box, TapFragranceAll other accessoriesRaw materialsDroppersBucketMachinesLabelsCosmetic accessoriesPouchesServicesPrinting servicesContract fillingPrinting servicesContract fillingNovelties	if(typeof(initedToggleMenu) == 'undfined') {
			var initedToggleMenu = false;
		}
		require(['jquery',
						'Ves_Megamenu/js/megamenuGeneral'
			],function($){
				if(typeof(playMegamenuJs) == 'function') {
					jQuery(document).ready(function($) {
						playMegamenuJs($, 'menu-top1531146492871782056', 3, 'hover', 0);
						
					});
				} else {
					jQuery(document).ready(function($) {
													$('.ves-drill-down-menu').find('.opener').addClass('ves-click');
							$(window).on('load resize',function(e){
								e.preventDefault();

								var back        	= '&lt;div class=&quot;hide-submenu&quot;>&lt;/div>';
								var subHide     	= $(back);
								var subMenu       	= $('.ves-drill-down-menu .submenu');
								
								// Add submenu hide bar
								if (subHide.children('hide-submenu').length ==0) {
									subHide.prependTo(subMenu);
								}
								var subHideToggle 	= $('.ves-drill-down-menu .hide-submenu');
								// Hide submenu
								subHideToggle.on(&quot;click&quot;, function() {
									$(this).parent().parent().parent().removeClass('view-submenu');
									$(this).parent().parent().parent().parent().parent().parent().parent().parent().removeClass('view-submenu');
									$(this).parent().hide();
								});

								if ($(window).width() &lt;= 768){
									
									$('.ves-drill-down-menu').find('.opener').addClass('fa fa-arrow-right').removeClass('opener');
									$('.ves-drill-down-menu').find('.navigation').addClass('navdrilldown').removeClass('navigation');
									$('.ves-drill-down-menu #menu-top1531146492871782056 .ves-click').on('click', function(e) {
										e.preventDefault();
										if ($(window).width() &lt;= 768){	
											
											$(this).removeClass('.item-active');
											$(this).parents('.submenu').addClass('view-submenu');
											$(this).parents('ul.ves-megamenu').addClass('view-submenu');
											var a = $(this).parents('li.nav-item').offset().top;
											var b = $(this).parents('ul.ves-megamenu').offset().top;
											var c = $(this).parent().parent().offset().top;

											$(this).parents('li.nav-item').children('.submenu').css('top',b-a+'px');
											$(this).parent().parent().children('.submenu').css('top',b-c+'px');
											$('.submenu.dropdown-menu').hide();
											$(this).parents('.submenu').show();
											$(this).parent().parent().children('.submenu').show();
											return false;

										}	
									});
								}else {
									
									$('.ves-drill-down-menu').find('.fa-arrow-right').addClass('opener').removeClass('fa fa-arrow-right');
									$('.ves-drill-down-menu').find('.navdrilldown').addClass('navigation').removeClass('navdrilldown');
								}
							});
								
							jQuery('#menu-top1531146492871782056-menu .ves-megamenu .level0').hover(function() {
								var mParentTop = jQuery(this).parents('.ves-megamenu').offset().top;
								var mParentHeight = $(this).parent().height();
								var mTop =  $(this).height();
								var mHeight = $(this).height();
								var mParent = $(this).parent();
								if (mHeight &lt; mParentHeight) {
									mTop = $(this).offset().top - mParent.offset().top + mHeight;
								}
								$(this).children('.submenu').css({top:mTop});	
							});
															jQuery('p').each(function() {
									var $this = $(this);
									if ($this.html().replace(/\s|&amp;nbsp;/g, '').length == 0)
									$this.remove();
								});
							});
					if(!initedToggleMenu) {
						var menuToogle = function () {
							if ($('html').hasClass('nav-open')) {
								$('html').removeClass('nav-open');
								setTimeout(function () {
									$('html').removeClass('nav-before-open');
								}, 300);
							} else {
								$('html').addClass('nav-before-open');
								setTimeout(function () {
									$('html').addClass('nav-open');
								}, 42);
							}
						}
						$(document).on(&quot;click&quot;, &quot;.action.nav-toggle&quot;, menuToogle);
						initedToggleMenu = true;
					}
					$(document).on(&quot;click&quot;, &quot;.ves-overlaymenu-top1531146492871782056&quot;, function(){
						$('#menu-top1531146492871782056').css(&quot;left&quot;,&quot;&quot;);
						$('html').removeClass('ves-navopen');
						setTimeout(function () {
							$('html').removeClass('ves-nav-before-open');
						}, 300);
						$(this).remove();
						return false;
					});

					$(&quot;#menu-top1531146492871782056 .dynamic-items li&quot;).hover(function(){
						$(this).parents(&quot;.dynamic-items&quot;).find(&quot;li&quot;).removeClass(&quot;dynamic-active&quot;);
						$(this).addClass(&quot;dynamic-active&quot;);
						var id = $(this).data(&quot;dynamic-id&quot;);
						$(&quot;#menu-top1531146492871782056 .&quot;+id).parent().find(&quot;.dynamic-item&quot;).removeClass(&quot;dynamic-active&quot;);
						$(&quot;#menu-top1531146492871782056 .&quot;+id).addClass(&quot;dynamic-active&quot;);
					});
					var mImg = '';
					$(&quot;#menu-top1531146492871782056 img&quot;).hover(function(){
						mImg = '';
						mImg = $(this).attr('src');
						if ($(this).data('hoverimg')){
							$(this).attr('src',$(this).data('hoverimg'));
						}
					},function(){
						$(this).attr('src',mImg);
					});
					$(&quot;#menu-top1531146492871782056 li a&quot;).hover(function(){
						$(this).css({
							&quot;background-color&quot;: $(this).data(&quot;hover-bgcolor&quot;),
							&quot;color&quot;: $(this).data(&quot;hover-color&quot;)
						});
					}, function(){
						$(this).css({
							&quot;background-color&quot;: $(this).data(&quot;bgcolor&quot;),
							&quot;color&quot;: $(this).data(&quot;color&quot;)
						});
					});
					$(window).on(&quot;resize load&quot;, function(){

						if($(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-bellow&quot;) &amp;&amp; $(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-above&quot;)){

							if (($(window).width() &lt;= $(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-bellow&quot;)) || ($(window).width()>= $(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-above&quot;))){
								$(&quot;#menu-top1531146492871782056-menu&quot;).hide();
							}else{
								$(&quot;#menu-top1531146492871782056-menu&quot;).show();
							}

							$(&quot;#menu-top1531146492871782056&quot;).find(&quot;li&quot;).each(function(index, element){
								if (($(window).width() &lt;= $(this).data(&quot;disable-bellow&quot;)) || ($(window).width()>= $(this).data(&quot;disable-above&quot;))){
									$(this).addClass(&quot;hidden&quot;);
								} else if ($(this).hasClass(&quot;hidden&quot;)){
									$(this).removeClass(&quot;hidden&quot;);
								}
							});

						} else if($(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-bellow&quot;) &amp;&amp; !$(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-above&quot;)) {
							if ($(window).width() &lt;= $(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-bellow&quot;)){
								$(&quot;#menu-top1531146492871782056-menu&quot;).hide();
							}else{
								$(&quot;#menu-top1531146492871782056-menu&quot;).show();
							}

							$(&quot;#menu-top1531146492871782056&quot;).find(&quot;li&quot;).each(function(index, element){
								if ($(window).width() &lt;= $(this).data(&quot;disable-bellow&quot;)){ $(this).addClass(&quot;hidden&quot;); }else if ($(this).hasClass(&quot;hidden&quot;)){ $(this).removeClass(&quot;hidden&quot;); } }); } else if($(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-above&quot;) &amp;&amp; !$(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-bellow&quot;)) { if ($(window).width()>= $(&quot;#menu-top1531146492871782056&quot;).data(&quot;disable-above&quot;)){
								$(&quot;#menu-top1531146492871782056-menu&quot;).hide();
							}else{
								$(&quot;#menu-top1531146492871782056-menu&quot;).show();
							}

							$(&quot;#menu-top1531146492871782056&quot;).find(&quot;li&quot;).each(function(index, element){
								if($(window).width() >= $(this).data(&quot;disable-above&quot;)) {
									$(this).addClass(&quot;hidden&quot;);
								} else if ($(this).hasClass(&quot;hidden&quot;)){
									$(this).removeClass(&quot;hidden&quot;);
								}
							});
						}
						
						if ($(window).width() >= 768 &amp;&amp; $(window).width() &lt;= 1024){ $(&quot;#menu-top1531146492871782056 .nav-anchor&quot;).off().click(function(){ var iParent=  $(this).parent('.nav-item'); iParent.addClass(&quot;clicked&quot;); if ($(iParent).children('.submenu').length==  1){ iParent.trigger('hover'); if (iParent.hasClass('submenu-alignleft') || iParent.hasClass('submenu-alignright')){ if ((iParent.offset().left + iParent.find('.submenu').eq(0).width())>$(window).width()){
											iParent.find('.submenu').eq(0).css('max-width','100%');
											iParent.css('position','static');
										}
									}
									return false;
								}
							});
						}else{
							$(&quot;#menu-top1531146492871782056&quot;).find('.submenu').css('max-width','');
							$(&quot;#menu-top1531146492871782056&quot;).find('.submenu-alignleft').css('position','relative');
						}
						if ($(window).width() &lt;= 768){
							$('.sections.nav-sections').removeAttr( &quot;style&quot; )
							$(&quot;#menu-top1531146492871782056&quot;).addClass(&quot;nav-mobile&quot;);

							/*
							$(&quot;#menu-top1531146492871782056 .nav-anchor&quot;).off().click(function(){

								$(&quot;#menu-top1531146492871782056 .nav-item&quot;).removeClass(&quot;item-active&quot;);
								var parent = $(this).parents(&quot;.nav-item&quot;).eq(0);
								$(this).toggleClass('item-active');
								$(parent).find(&quot;.submenu&quot;).eq(0).slideToggle();
								return false;
							});
							*/

						}else{
							$(&quot;#menu-top1531146492871782056&quot;).find(&quot;.submenu&quot;).css({'display':''});
							$(&quot;#menu-top1531146492871782056&quot;).find(&quot;div&quot;).removeClass(&quot;mbactive&quot;);
							$(&quot;#menu-top1531146492871782056&quot;).removeClass(&quot;nav-mobile&quot;);

							/*
							$(&quot;#menu-top1531146492871782056 .nav-anchor&quot;).off().click(function(){
								if(&quot;#&quot; != $(this).attr(&quot;href&quot;))
									window.location.url = $(this).attr(&quot;href&quot;);
								return true;
							});
							*/

						}
					}).resize();
					//Toggle mobile menu
					$('.ves-megamenu-mobile #menu-top1531146492871782056 .opener').on('click', function(e) {
						e.preventDefault();
						$(&quot;#menu-top1531146492871782056 .nav-item&quot;).removeClass(&quot;item-active&quot;);
						var parent = $(this).parents(&quot;.nav-item&quot;).eq(0);
						$(this).toggleClass('item-active');
						$(parent).find(&quot;.submenu&quot;).eq(0).slideToggle();
						return false;
					});

					
											$(document).ready(function(){
							$('header.page-header .container_navigation ul li.dropdown').on('mouseover', function() {
								   $('.mega_menu').hide();
								   $(this).find('.mega_menu').show();
								});
							   $('html').click(function() {
							   $(this).find('.mega_menu').hide();
							   });
							 });
		
							$('.mega_menu').click(function(event){
								   event.stopPropagation();
							});
									}
			});
        
        
 
 
 

//&lt;![CDATA[

requirejs(['jquery', 'smartheader', 'stickyheader'], function(jQuery, smartheader, stickyheader) {

    var theHeaderContainer = jQuery('#header-container');

        
        //alert('header tpl, before smartheader'); ///TODO

        theHeaderContainer.smartheader();

    
    jQuery(function($) {

        //console.log('header tpl, on(ready), ater smartheader'); ///
        //alert('header tpl, on(ready), ater smartheader'); ///TODO

        
            //Skip Links
            var skipContents = $('.skip-content');
            var skipLinks = $('.skip-link');
            skipLinks.on('click', function (e) {
                e.preventDefault();

                var self = $(this);
                var target = self.attr('href');

                //Get target element
                var elem = $(target);

                //Check if stub is open
                var isSkipContentOpen = elem.hasClass('skip-active') ? 1 : 0;

                //Hide all stubs
                skipLinks.removeClass('skip-active');
                skipContents.removeClass('skip-active');

                //Toggle stubs
                if (isSkipContentOpen) {
                    self.removeClass('skip-active');
                } else {
                    self.addClass('skip-active');
                    elem.addClass('skip-active');
                }
            });

        
        
            var stickyHeaderSettings = {
                stickyThreshold: 992            };
            theHeaderContainer.stickyheader(stickyHeaderSettings);

        
    }); //end: on document ready

}); //end: requirejs

//]]>


//&lt;![CDATA[

    //Expose the header container if jQuery script (smartheader) failed

    // var jsHeaderContainerObject = document.getElementById(&quot;header-container&quot;);
    // if (jsHeaderContainerObject.style.display == 'none')
    // {
    //     jsHeaderContainerObject.style.display = &quot;block&quot;;
    //     jsHeaderContainerObject.classList.add(&quot;js-shown&quot;); ///
    // }

//]]>




    
    



    
            
    
            

                
                    
                                                    


 





 














                                            
                 
                
             
                 

//&lt;![CDATA[
requirejs(['jquery','owlcarousel'], function(jQuery, owlcarousel){ //

    jQuery(function($) {
        
        var owl = $('#slideshow-354e58af5ee405609331cfc73f8c003c');
        owl.owlCarousel({

            navigationText: false
            , addClassActive: true

                    , singleItem: true

            
        
                    , slideSpeed: 200        
                    , paginationSpeed: 500        
                    , autoPlay: 10000        
                    , stopOnHover: true
        
                    , rewindNav: true
            , rewindSpeed: 600
        
                    , pagination: true
        
                    , navigation: true
        
        
            , beforeInit: function() {
                var firstSlide = owl.find('.item:eq(0)');
                firstSlide.find('[data-animate-in]').each(function() {
                    $(this).addClass($(this).data('animate-in') + ' animated');
                });
            }

                        , afterMove: function() {
                owl.find('.owl-item.active [data-animate-in]').each(function() {
                    $(this).addClass($(this).data('animate-in') + ' animated');
                });
            }

            , beforeMove: function() {
                owl.find('.owl-item.active [data-animate-in]').each(function() {
                    $(this).removeClass($(this).data('animate-in') + ' animated');
                });
            }

        
        }); // end: owl

    });

});
//]]>


    
        window.authenticationPopup = {&quot;autocomplete&quot;:&quot;off&quot;,&quot;customerRegisterUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/customer\/account\/create\/&quot;,&quot;customerForgotPasswordUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/customer\/account\/forgotpassword\/&quot;,&quot;baseUrl&quot;:&quot;http:\/\/uk.nordtek.dev.erise.hu\/&quot;};
    
    



    






    

Fast Shipping


Quantity discounts on every product


Free shipping on orders over €200


Free product samples




    Packaging categories
Jar Bottle Pumps Glass products Tube Sets Trigger sprayer Airless Cardboard boxes Sprayers Foam pump Labels Cap Cosmetic Ingredients

show more
show less


    Latest customer reviews

        
        
            
        
        
            Pumpa, 28/410, DPGR, fehér, FBOG 160 mm, K2 aktuator
            
                
                    Overall review
                    
                                                
                                                
                                                
                                                
                                                                        
                                            
                
                
                    Posted:
                    2018-07-04
                
                
                    Reviewer:
                    Tóth József
                
            
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vestibulum commodo tincidunt. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.

Vestibulum volutpat pharetra arcu, eu consectetur dolor semper pharetra. Proin in pharetra orci, nec fermentum enim. Duis eu commodo velit. In vel mauris ullamcorper, tincidunt libero in, sodales enim.
Cras consectetur tortor lacus, a fringilla nibh blandit quis. 
        
    
        
        
            
        
        
            Üveg, DIN 18, 30 ml, barna, gyógyszeripari
            
                
                    Overall review
                    
                                                
                                                
                                                
                                                                        
                                                
                                            
                
                
                    Posted:
                    2018-07-06
                
                
                    Reviewer:
                    Tóth József
                
            
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sodales auctor libero ut interdum. Nulla faucibus interdum ex, non egestas augue tempus id. Sed sed vulputate magna, non sodales urna.
Vestibulum sit amet elementum neque. Nullam id nisi commodo, euismod arcu sit amet, pretium mauris. Nullam vulputate blandit lacus, at dictum sem rutrum id. Donec vel metus mauris.

Duis cursus, nunc sit amet porta pharetra, nunc turpis volutpat libero, vitae pellentesque nulla neque vitae lorem.
        
    
    


        Featured Articles
    
                
            
                
                    Read more
                
            
            250 ml Transparent Jar for Face Masks
        
                
            
                
                    Read more
                
            
            DIY Avocado Skin Care Cream Recipe
        
                
            
                
                    Read more
                
            
            DIY Face Mask Labeling Tips
        
                
            
                
                    Read more
                
            
            Glass Containers’ Utility For Food Storage In Your Kitchen
        
                
            
                
                    Read more
                
            
            Importance of Product Information In Packaging Labels
        
                
            
                
                    Read more
                
            
            Most Important Packaging Features for Consumers
        
            


    

Buy packaging supplies easier with All in Packaging!



&lt;!--
.ic-arrow-right {color: #2ebebb;}
-->

Simple online procurement
Thousands of products and services at one place
Complex packaging solutions
Extensive stock




Low minimum order quantities
Personalized service
Fast delivery
Reliable supplier



With All in Packaging you can get the best packaging for your product. Our portfolio contains thousands of packaging products such as bottles, jars, caps, sprayers, tubes, cardboard boxes and others. You can choose from our portfolio or get a uniqe design. We can help you implement any custom packaging ideas. Related services like labeling and contract filling are also available at the same place.
Are you manufacturing or distributing packagings?
Using All in Packaging you can reach both large customers and small quantity buyers. All In Packaging is a smart solution for both packaging manufacturers and distributors, where manufacturers can easily reach an international customer base via the All In Packaging sales network, and distributors can support their sales with modern turn-key e-commerce solutions.

Partnering with All In Packaging is simple. Join All In Packaging and benefit from our sales- and marketing efforts!
Learn more our Partner Program!

WHAT DO YOU NEED?

 Cosmetics 
 Household chemicals 
 Car care 
 Food packaging 
 Gardening 
 Pharmaceutical packaging 





    
  On sale
  What's new
  Featured products



  
          

    Products on sale
    
        
            
                                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    Kupak, 24/410, flip-top, hengeres, egyedi barna, kiöntőnyílás 2,5mm                                
                            

                            
                            
    
        


            Regular Price
        
        €0.1402    
            

    
        €0.1168
        
    
    
        


            Now Only
        
        €0.1080    
            

    
        €0.0900
        
    

            
            


            As low as
        
        €0.0714    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    SEKIPACK SLR250 250 ml hideg étel, PET 1 zsák (60 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €7.0915    
            

    
        €5.9096
        
    
    
        


            Now Only
        
        €3.6000    
            

    
        €3.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ONDIPACK OK250 250 ml meleg étel, PP 1 zsák (50 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €5.8240    
            

    
        €4.8533
        
    
    
        


            Now Only
        
        €2.4000    
            

    
        €2.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ONDIPACK OK375 375 ml meleg étel, PP 1 zsák (50 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €6.2348    
            

    
        €5.1957
        
    
    
        


            Now Only
        
        €2.4000    
            

    
        €2.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ONDIPACK OK500 500 ml meleg étel, PP 1 zsák (50 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €8.4144    
            

    
        €7.0120
        
    
    
        


            Now Only
        
        €3.6000    
            

    
        €3.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ONDIPACK OK750 750 ml meleg étel, PP 1 zsák (50 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €14.3341    
            

    
        €11.9451
        
    
    
        


            Now Only
        
        €6.0000    
            

    
        €5.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    GASTROPACK GN6H40 750 ml fehér meleg étel, fóliázható PP 1 zsák (100 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €20.1600    
            

    
        €16.8000
        
    
    
        


            Now Only
        
        €8.4000    
            

    
        €7.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    GASTROPACK GN6H402C 2 x 327 ml fehér meleg étel, fóliázható PP 1 zsák (100 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €20.8200    
            

    
        €17.3500
        
    
    
        


            Now Only
        
        €8.4000    
            

    
        €7.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    GASTROPACK GN1/4H55B 1500 ml fehér meleg étel, fóliázható PP 1 zsák (85 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €29.4960    
            

    
        €24.5800
        
    
    
        


            Now Only
        
        €12.0000    
            

    
        €10.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ALPHACEL CL250TPE 250 ml meleg étel, fóliázható PP 1 zsák (100 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €14.6400    
            

    
        €12.2000
        
    
    
        


            Now Only
        
        €2.4000    
            

    
        €2.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ALPHACEL CL650TPE 640 ml meleg étel, fóliázható PP 1 zsák (140 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €12.5993    
            

    
        €10.4994
        
    
    
        


            Now Only
        
        €6.0000    
            

    
        €5.0000
        
    


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        Sale
                                                                                                                    
                         
                        
                                                        
                                
                                    ALPHACEL CL750TPE 730 ml meleg étel, fóliázható PP 1 zsák (100 db)                                
                            

                            
                            
    
        


            Regular Price
        
        €11.1600    
            

    
        €9.3000
        
    
    
        


            Now Only
        
        €4.8000    
            

    
        €4.0000
        
    


                            

                                
                             
                         
                     
                
                                
                                
                                
                                
                                
                                
                                
                                
                                
                                
                                
                             
             
     

    

//&lt;![CDATA[

    requirejs(['jquery','owlcarousel'], function(jQuery, owl)
    {
        jQuery(function($) {
            var owl = $('#itemslider-featured-497212b586d973cf1315b79956f04f97');
            owl.owlCarousel({

                                    lazyLoad: true,
                 
                                    itemsCustom: [ [0, 1], [320, 2], [480, 3], [768, 4], [992, 5], [1200, 6] ],
                    responsiveRefreshRate: 50,
                 
                                    slideSpeed: 200,
                 
                                    paginationSpeed: 500,
                 
                                    scrollPerPage: true,
                 
                 
                                    stopOnHover: true,
                 
                                    rewindNav: true,
                    rewindSpeed: 600,
                 
                                    pagination: false,
                                    navigation: true,
                    navigationText: false

            });
        });
    });

//]]>


  
  
         
  
  
          

    Featured products
    
        
            
                                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    SET-Cseppentő flakon, 5 ml, átlátszó, menetes, üveg+Gumi cseppentőfej, fehér, ezüst palásttal                                
                            

                            
                            
    


        
        €0.4860    
            

    
        €0.4050
        

            
            


            As low as
        
        €0.5024    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Öblítős kupak, 7 gr, arany                                
                            

                            
                            
    


        
        €0.1346    
            

    
        €0.1122
        

            
            


            As low as
        
        €0.0686    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Hengeres 500ml PE flakon, fehér                                
                            

                            
                            
    


        
        €1.1160    
            

    
        €0.9300
        

            
            


            As low as
        
        €0.5690    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Garanciazáras kupak 500ml-s flakonhoz                                
                            

                            
                            
    


        
        €0.3778    
            

    
        €0.3148
        

            
            


            As low as
        
        €0.1927    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    5 literes Politainer kannához átöntő                                
                            

                            
                            
    


        
        €0.4886    
            

    
        €0.4072
        

            
            


            As low as
        
        €0.2492    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    100 ml PET flakon, barna 18/410                                
                            

                            
                            
    


        
        €0.2952    
            

    
        €0.2460
        

            
            


            As low as
        
        €0.1506    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    750 ml PP füles flakon, natúr                                
                            

                            
                            
    


        
        €1.0147    
            

    
        €0.8456
        

            
            


            As low as
        
        €0.5176    
        
        
    
                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Cap, flip top, 35mm, PP, blue                                
                            

                            
                            
    


        
        €0.0000    
            

    
        €0.0000
        


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Screw cap, blue, neck 32 mm, PP                                
                            

                            
                            
    


        
        €0.0000    
            

    
        €0.0000
        


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Aloe                                
                            

                            
                            
    


        
        €0.0000    
            

    
        €0.0000
        


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Cherry                                
                            

                            
                            
    


        
        €0.0000    
            

    
        €0.0000
        


                            

                                
                             
                         
                     
                
                    
                        

                            
                                
                                                                    
                                
                                                                
                            

                                                        
                                                                                                                    
                         
                        
                                                        
                                
                                    Coconut fantasy                                
                            

                            
                            
    


        
        €0.0000    
            

    
        €0.0000
        


                            

                                
                             
                         
                     
                
                                
                                
                                
                                
                                
                                
                                
                                
                                
                                
                                
                             
             
     

    

//&lt;![CDATA[

    requirejs(['jquery','owlcarousel'], function(jQuery, owl)
    {
        jQuery(function($) {
            var owl = $('#itemslider-featured-a3c7627e8b07e33903f997ccb0431159');
            owl.owlCarousel({

                                    lazyLoad: true,
                 
                                    itemsCustom: [ [0, 1], [320, 2], [480, 3], [768, 4], [992, 5], [1200, 6] ],
                    responsiveRefreshRate: 50,
                 
                                    slideSpeed: 200,
                 
                                    paginationSpeed: 500,
                 
                                    scrollPerPage: true,
                 
                 
                                    stopOnHover: true,
                 
                                    rewindNav: true,
                    rewindSpeed: 600,
                 
                                    pagination: false,
                                    navigation: true,
                    navigationText: false

            });
        });
    });

//]]>


  



    
    Sign up to our Newsletters!
    
        
            
                
            
            
                
            
        
        
            
                I want special offers!
                
            
        
    






    
    
        
        
    
        
            

                
                                    
                        

Customer support


Shipping
Dropshipping
Wholesale
How to send an order
How to request free sample






GTC


About pricing policy
General terms and conditions
Label terms and conditions
General cupon terms
Privacy statement






About us


All In Packaging
Our references
Partner program
Previous newsletters






Contact us


Nordtek Packaging Ltd.


30 worthing road Horsham RH12 1SL United Kingdom


+44 14 03 887 124 info@allinpackaging.co.uk



                    
                                
                            
             
         
    
    
    
    
    
    
        
            

                
                    
                        © 1997-2018 Nordtek Packaging Ltd.
                        All rights reserved
                    
                

                
                    
                        
                            
                        
                        
                            
                        
                        
                            
                        
                        
                            
                        
                    
                

                
                    
                        
                    
                

                
                    
                        
                    
                

                
                    
                        
                    
                

             
         
    

        
    

 
 
 



    
    
    



	require([
		'jquery'], function ($) {
			$(document).ready(function(){
				if(jQuery(&quot;.megamenuowl-play&quot;).length > 0 ) {
					require([
						'jquery',
												&quot;Ves_All/lib/bootstrap/js/bootstrap.min&quot;,
															'megamenuowlcarousel'
								], function ($) {
					var owlItems = [];
					jQuery(&quot;.megamenuowl-play&quot;).each( function(){
						var owlCarousel = jQuery(this).find(&quot;.owl-carousel&quot;);
						var owlId = jQuery(owlCarousel).attr(&quot;id&quot;);
						jQuery(this).addClass(&quot;hasOwl&quot;);
						var config = [];
						owlItems[jQuery(owlCarousel).attr(&quot;id&quot;)] = true;
						if(typeof(jQuery(owlCarousel).data('nav'))!='underfined'){
							config['nav'] = jQuery(owlCarousel).data('nav');
						}
						if(typeof(jQuery(owlCarousel).data('dot'))!='underfined'){
							config['dot'] = jQuery(owlCarousel).data('dot');
						}
						if(typeof(jQuery(owlCarousel).data('autoplay'))!='underfined'){
							config['autoplay'] = jQuery(owlCarousel).data('autoplay');
						}
						if(jQuery(owlCarousel).data('autoplay-timeout')){
							config['autoplayTimeout'] = jQuery(owlCarousel).data('autoplay-timeout');
						}
						if(jQuery(owlCarousel).data('autoplay-pauonhover')){
							config['autoplayHoverPause'] = jQuery(owlCarousel).data('autoplay-pauonhover');
						}
						if(typeof(jQuery(owlCarousel).data('rtl'))!='underfined'){
							config['rtl'] = jQuery(owlCarousel).data('rtl');
						}
						if(jQuery(owlCarousel).data('items')){
							
							config['items'] = jQuery(owlCarousel).data('items');
						}
						if(typeof(jQuery(owlCarousel).data('loop'))!='underfined'){
							config['loop'] = jQuery(owlCarousel).data('loop');
						}
						if(typeof(jQuery(owlCarousel).data('mousedrag'))!='underfined'){
							config['mouseDrag'] = jQuery(owlCarousel).data('mousedrag');
						}
						if(typeof(jQuery(owlCarousel).data('pulldrag'))!='underfined'){
							config['pullDrag'] = jQuery(owlCarousel).data('pulldrag');
						}
						if(typeof(jQuery(owlCarousel).data('freedrag'))!='underfined'){
							config['freeDrag'] = jQuery(owlCarousel).data('freedrag');
						}
						if(typeof(jQuery(owlCarousel).data('stagepadding'))!='underfined'){
							config['stagePadding'] = jQuery(owlCarousel).data('stagepadding');
						}
						if(typeof(jQuery(owlCarousel).data('lazyload'))!='underfined'){
							config['lazyLoad'] = jQuery(owlCarousel).data('lazyload');
						}
						if(jQuery(owlCarousel).data('margin')){
							config['margin'] = jQuery(owlCarousel).data('margin');
						}
						var mobile_items = 1;
						if(jQuery(owlCarousel).data('mobile-items')){
							mobile_items = jQuery(owlCarousel).data('mobile-items');
						}
						var tablet_small_items = 3;
						if(jQuery(owlCarousel).data('tablet-small-items')){
							tablet_small_items = jQuery(owlCarousel).data('tablet-small-items');
						}
						var tablet_items = 3;
						if(jQuery(owlCarousel).data('tablet-items')){
							tablet_items = jQuery(owlCarousel).data('tablet-items');
						}
						var portrait_items = 4;
						if(jQuery(owlCarousel).data('portrait-items')){
							portrait_items = jQuery(owlCarousel).data('portrait-items');
						}
						var large_items = 5;
						if(jQuery(owlCarousel).data('large-items')){
							large_items = jQuery(owlCarousel).data('large-items');
						}
						var large_max_items = 6;
						if(jQuery(owlCarousel).data('large-max-items')){
							large_max_items = jQuery(owlCarousel).data('large-max-items');
						}
						config['responsive'] = {
							0 : {items: mobile_items},
							480 : {items: tablet_small_items},
							640 : {items: tablet_items},
							768 : {items: portrait_items},
							980 : {items: large_items},
							1200 : {items: large_max_items}
						};
						jQuery(owlCarousel).owlCarousel(config);
						jQuery(&quot;.owl-left&quot;).click(function(){
							var owlCarousel = jQuery(&quot;#&quot;+jQuery(this).data(&quot;owlid&quot;));
							owlCarousel.trigger('prev.owl.carousel');
							return false;
						});
						jQuery(&quot;.owl-right&quot;).click(function(){
							var owlCarousel = jQuery(&quot;#&quot;+jQuery(this).data(&quot;owlid&quot;));
							owlCarousel.trigger('next.owl.carousel');
							return false;
						});
					});
				});
}
})
});


//&lt;![CDATA[

requirejs(['jquery'], function(jQuery) {

        


    jQuery(function($) {



        // Products grid: equal height of items
        


                
            var startHeight;
            var startPaddingBottom;
            $('.category-products-grid').on('mouseenter', '.item', function() {

                    var $item = $(this);

                                                        if ($(window).width() >= 320)
                    {
                
                    
                    var bottomMinSpace = 20;
                    var paddingBottom2 = 0;
                    var $actionsBlock = $item.find('.actions');

                    startHeight = $item.height();
                    startPaddingBottom = parseInt($item.css(&quot;padding-bottom&quot;));

                    $item.css(&quot;height&quot;, &quot;auto&quot;); // Reset height
                    $item.find(&quot;.display-onhover&quot;).fadeIn(400, &quot;easeOutCubic&quot;); // Show elements visible on hover
                    var h2 = $item.height();
                    
                    // -------------------------------------------------------------------------
                    // Compare start padding with new on-hover padding, calculate the difference

                    // Get actions height and calculate new padding
                    // Calculate new bottom padding wich equals to: actions container height + bottomMinSpace
                    paddingBottom2 = bottomMinSpace + $actionsBlock.innerHeight();

                    // Calculate difference between start padding and new padding
                    var paddingBottomDiff = paddingBottom2 - startPaddingBottom;

                    // Apply only if new padding is larger than start padding
                    if (paddingBottomDiff > 0)
                    {
                        $item.css(&quot;padding-bottom&quot;, paddingBottom2 + &quot;px&quot;);
                    }

                    // -------------------------------------------------------------------------
                    // Compare start height with new (on-hover) height, calculate the difference.
                    // Important: new height includes difference between start padding and new padding
                    var diff = 0;
                    if (h2 &lt; startHeight)
                    {
                        $item.height(startHeight);
                    }
                    else
                    {
                        $item.height(h2);
                        diff = h2 - startHeight;
                        if (paddingBottomDiff > 0)
                        {
                            diff += paddingBottomDiff;
                        }
                    }
                    
                    // -------------------------------------------------------------------------
                    // Apply height difference as nagative margin, but only if new height
                    // is larger than start height.
                    if (diff > 0)
                    {
                        $item.css(&quot;margin-bottom&quot;, &quot;-&quot; + diff + &quot;px&quot;);
                    }

                                    }                                 
            }).on('mouseleave', '.item', function() {

                    var $item = $(this);

                                                    if ($(window).width() >= 320)
                    {
                
                    // Clean up
                    $item.find(&quot;.display-onhover&quot;).stop(true).hide();
                    $item.css(&quot;margin-bottom&quot;, &quot;&quot;);

                                                                $item.css(&quot;height&quot;, &quot;&quot;);
                        $item.css(&quot;padding-bottom&quot;, &quot;&quot;);
                    
                                    }                                 
            });
        
        


                $('.products-grid, .products-list').on('mouseenter', '.product-item-img', function() {
            $(this).find(&quot;.alt-img&quot;).fadeIn(400, &quot;easeOutCubic&quot;);
        }).on('mouseleave', '.product-item-img', function() {
            $(this).find(&quot;.alt-img&quot;).stop(true).fadeOut(400, &quot;easeOutCubic&quot;);
        });



                $('.fade-on-hover').on('mouseenter', function() {
            $(this).animate({opacity: 0.75}, 300, 'easeInOutCubic');
        }).on('mouseleave', function() {
            $(this).stop(true).animate({opacity: 1}, 300, 'easeInOutCubic');
        });



        // Drop-down
        var ddBlockSelector = '.dropdown-block';
        var ddOpenTimeout;
        var dMenuPosTimeout;
        var DD_DELAY_IN = 200;
        var DD_DELAY_OUT = 0;
        var DD_ANIMATION_IN = 0;
        var DD_ANIMATION_OUT = 0;

        $(document).on('mouseenter touchstart', ddBlockSelector, function(e) {

            var dd = $(this);
            var ddHeading = dd.children('.dropdown-heading');
            var ddContent = dd.children('.dropdown-content');

            // If dd is not opened yet (or not initialized yet)
            var isDdOpened = dd.data('ddOpened');
            if (isDdOpened === false || isDdOpened === undefined)
            {
                // Clear old position of dd menu
                ddContent.css(&quot;left&quot;, &quot;&quot;);
                ddContent.css(&quot;right&quot;, &quot;&quot;);

                // Show dd menu
                clearTimeout(ddOpenTimeout);
                ddOpenTimeout = setTimeout(function() {
                    
                    dd.addClass('open');
                    dd.data('ddOpened', true);
                    ddContent.promise().done(function() {
                        dd.trigger('dropdown-block-opened');
                    });
                    
                }, DD_DELAY_IN);

                ddContent.stop(true, true).delay(DD_DELAY_IN).fadeIn(DD_ANIMATION_IN, &quot;easeOutCubic&quot;);
                
                // Set new position of dd menu.
                // This code is delayed the same amount of time as dd animation.
                clearTimeout(dMenuPosTimeout);
                dMenuPosTimeout = setTimeout(function() {

                    if (ddContent.offset().left &lt; 0)
                    {
                        var space = dd.offset().left; // Space available on the left of dd
                        ddContent.css(&quot;left&quot;, (-1)*space);
                        ddContent.css(&quot;right&quot;, &quot;auto&quot;);
                    }
                
                }, DD_DELAY_IN);

            } // end: dd is not opened yet

        }).on('mouseleave', ddBlockSelector, function(e) {

            var dd = $(this);
            var ddContent = dd.children('.dropdown-content');

            clearTimeout(ddOpenTimeout); // Clear, to close dd on mouseleave
            ddContent.stop(true, true).delay(DD_DELAY_OUT).fadeOut(DD_ANIMATION_OUT, &quot;easeInCubic&quot;);
            if (ddContent.is(&quot;:hidden&quot;))
            {
                ddContent.hide();
            }
            dd.removeClass('open');

            // Clear dd open flag
            dd.data('ddOpened', false);

            // After hiding, clear the click event flag
            dd.data('ddClickIntercepted', false);

        }).on('click', ddBlockSelector, function(e) {

            var dd = $(this);
            var ddHeading = dd.children('.dropdown-heading');
            var ddContent = dd.children('.dropdown-content');

            // Only if the heading was clicked
            if ($.contains(ddHeading[0], e.target) || ddHeading.is(e.target))
            {
                // Only after the first click already happened, the second click can close the dropdown
                if (dd.data('ddClickIntercepted'))
                {
                    if (dd.hasClass('open'))
                    {
                        clearTimeout(ddOpenTimeout); // Clear, to close dd on mouseleave
                        ddContent.stop(true, true).delay(DD_DELAY_OUT).fadeOut(DD_ANIMATION_OUT, &quot;easeInCubic&quot;);
                        if (ddContent.is(&quot;:hidden&quot;))
                        {
                            ddContent.hide();
                        }
                        dd.removeClass('open');

                        // Clear dd open flag
                        dd.data('ddOpened', false);

                        // After hiding, clear the click event flag
                        dd.data('ddClickIntercepted', false);
                    }
                }
                else 
                {
                    // Set the click event flag
                    dd.data('ddClickIntercepted', true);
                }
            }

        });



        // Back to top
        var windowScroll_t;
        $(window).scroll(function(){
            
            clearTimeout(windowScroll_t);
            windowScroll_t = setTimeout(function() {
                                        
                if ($(this).scrollTop() > 100)
                {
                    $('#scroll-to-top').fadeIn();
                }
                else
                {
                    $('#scroll-to-top').fadeOut();
                }
            
            }, 500);
            
        });
        
        $('#scroll-to-top').click(function(){
            $(&quot;html, body&quot;).animate({scrollTop: 0}, 600, &quot;easeOutCubic&quot;);
            return false;
        });



                var dResize = {

            winWidth : 0
            , winHeight : 0
            , windowResizeTimeout : null

            , init : function()
            {
                dResize.winWidth = $(window).width();
                dResize.winHeight = $(window).height();
                dResize.windowResizeTimeout;

                $(window).on('resize', function(e) {
                    clearTimeout(dResize.windowResizeTimeout);
                    dResize.windowResizeTimeout = setTimeout(function() {
                        dResize.onEventResize(e);
                    }, 50);
                });
            }

            , onEventResize : function(e)
            {
                //Prevent from executing the code in IE when the window wasn't actually resized
                var winNewWidth = $(window).width();
                var winNewHeight = $(window).height();

                //Code in this condition will be executed only if window was actually resized
                if (dResize.winWidth != winNewWidth || dResize.winHeight != winNewHeight)
                {
                    //Trigger deferred resize event
                    $(window).trigger(&quot;themeResize&quot;, e);

                    //Additional code executed on deferred resize
                    dResize.onEventDeferredResize();
                }

                //Update window size variables
                dResize.winWidth = winNewWidth;
                dResize.winHeight = winNewHeight;
            }

            , onEventDeferredResize : function() //Additional code, execute after window was actually resized
            {

                //Products grid: equal height of items
                
            }

        }; //end: dResize

        dResize.init();



    }); //end: on document ready



    jQuery(window).on('load',function() {

                
    }); //end: on load



}); //end: requirejs



requirejs(['jquery', 'enquire'], function(jQuery, enquire) {

    jQuery(function($) {

        // Collapsible block
        // This plugin requires a specific markup structure. The plugin expects a set of elements that it
        // will use as the toggle link. It then hides all immediately following siblings and toggles the sibling's
        // visibility when the toggle link is clicked.
        //
        // Example markup:
        // &lt;div class=&quot;block&quot;>
        //     &lt;div class=&quot;block-title&quot;>Trigger&lt;/div>
        //     &lt;div class=&quot;block-content&quot;>Content that should show when &lt;/div>
        // &lt;/div>
        //
        // JS: jQuery('.block-title').toggleSingle();
        //
        // Options:
        //     destruct: defaults to false, but if true, the plugin will remove itself, display content, and remove event handlers

        jQuery.fn.toggleSingle = function (options) {

            // passing destruct: true allows
            var settings = $.extend({
                destruct: false
            }, options);

            return this.each(function () {
                if (!settings.destruct) {
                    $(this).on('click', function () {
                        $(this)
                            .next()
                            .toggleClass('no-display')
                            .parent()
                            .toggleClass('active');
                            // .toggleClass('active')
                            // .next()
                            // .toggleClass('no-display');
                    });
                    // Hide the content
                    $this = $(this);
                    if (!$this.parent().hasClass('active'))
                    {
                        $this.next().addClass('no-display');
                    }
                }
                else
                {
                    // Remove event handler so that the toggle link can no longer be used
                    $(this).off('click');
                    // Remove all classes that were added by this plugin
                    $(this)
                        .next()
                        .removeClass('no-display')
                        .parent()
                        .removeClass('active');
                }

            });

        } // end: toggleSingle

        var breakpointScreenM = 768; // The same value as Magento's breakpoint @screen__m

        // Blocks collapsing on smaller viewports
        enquire.register('(max-width: ' + (breakpointScreenM - 1) + 'px)', {
            setup: function () {
                this.toggleElements = $(
                    '.sidebar .block:not(#layered-filter-block) .block-title, ' +
                    '.mobile-collapsible .block-title'
                );
            },
            match: function () {
                this.toggleElements.toggleSingle();
            },
            unmatch: function () {
                this.toggleElements.toggleSingle({destruct: true});
            }
        });

        // Blocks collapsing on all viewports.
        // For backward compatibility exclude blocks which have both classes: &quot;collapsible&quot; and &quot;mobile-collapsible&quot; 
        $('.collapsible:not(.mobile-collapsible) .block-title').toggleSingle();

    }); //end: on document ready

}); //end: requirejs



//]]>

    

/html[1]


    
    
        
            
            
                Close
            
        
        
    
        
            Checkout as a new customer
        
        
            Creating an account has many benefits:
            
                See order and shipping status
                Track order history
                Check out faster
            
            
                
                    
                        Sign Up
                    
                
            
        
    

    
        
            Checkout using your account
        
        
        

    
    


        
        
        
            
                
                    
                        Email Address
                        
                            
                        
                    
                    
                        Password
                        
                            
                        
                    
                    
                    


                    
                    
                        
                        
                            
                                Sign In
                            
                        
                        
                            
                                Forgot Your Password?
                            
                        
                    
                
            
        
    

        
    
    
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
